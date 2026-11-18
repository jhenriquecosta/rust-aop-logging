use std::time::Duration;

/// Item de um pedido, com preço unitário em centavos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Item {
    pub unit_price_cents: u64,
    pub quantity: u32,
}

/// Pedido de entrada do pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub customer_id: u64,
    pub items: Vec<Item>,
}

/// Resposta calculada pelo pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Response {
    pub customer_name: String,
    pub total_cents: u64,
    pub items_count: u32,
}

/// Falhas de negócio do pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineError {
    EmptyOrder,
    InvalidQuantity,
    CustomerNotFound,
}

impl std::fmt::Display for PipelineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineError::EmptyOrder => write!(f, "pedido sem itens"),
            PipelineError::InvalidQuantity => write!(f, "quantidade inválida"),
            PipelineError::CustomerNotFound => write!(f, "cliente não encontrado"),
        }
    }
}

impl std::error::Error for PipelineError {}

/// Atraso simulado em cada chamada de IO (representa latência de rede).
pub const IO_DELAY: Duration = Duration::from_millis(20);

/// Cliente do pedido, com nome e indicador de VIP.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Customer {
    pub name: String,
    pub vip: bool,
}

/// Pedido persistido pelo repositório.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Order {
    pub customer: Customer,
    pub total_cents: u64,
    pub items_count: u32,
}

/// Cliente de cadastro de clientes (simulado, com IO).
#[derive(Debug, Clone)]
pub struct CustomerClient {
    pub delay: Duration,
}

impl CustomerClient {
    pub fn new(delay: Duration) -> Self {
        Self { delay }
    }

    #[tracing::instrument(name = "load_customer", skip(self))]
    pub async fn load(&self, id: u64) -> Result<Customer, PipelineError> {
        tokio::time::sleep(self.delay).await;
        if id == 404 {
            return Err(PipelineError::CustomerNotFound);
        }
        Ok(Customer {
            name: format!("Cliente {id}"),
            vip: id % 2 == 0,
        })
    }
}

/// Repositório de pedidos (simulado, com IO).
#[derive(Debug, Clone)]
pub struct OrderRepository {
    pub delay: Duration,
}

impl OrderRepository {
    pub fn new(delay: Duration) -> Self {
        Self { delay }
    }

    #[tracing::instrument(name = "persist", skip(self, _order))]
    pub async fn persist(&self, _order: &Order) -> Result<(), PipelineError> {
        tokio::time::sleep(self.delay).await;
        Ok(())
    }
}

fn validate_impl(req: &Request) -> Result<(), PipelineError> {
    if req.items.is_empty() {
        return Err(PipelineError::EmptyOrder);
    }
    if req.items.iter().any(|item| item.quantity == 0) {
        return Err(PipelineError::InvalidQuantity);
    }
    Ok(())
}

fn price_impl(req: &Request) -> u64 {
    req.items
        .iter()
        .map(|item| item.unit_price_cents * u64::from(item.quantity))
        .sum()
}

fn apply_rules_impl(subtotal: u64, customer: &Customer) -> u64 {
    if customer.vip {
        subtotal * 9 / 10
    } else {
        subtotal
    }
}

fn items_count_impl(req: &Request) -> u32 {
    req.items.iter().map(|item| item.quantity).sum()
}

/// Executa o pipeline completo: valida, carrega o cliente, precifica,
/// aplica as regras, persiste e monta a resposta.
#[tracing::instrument(skip(req))]
pub async fn handle(req: Request) -> Result<Response, PipelineError> {
    validate_impl(&req)?;

    let customer = CustomerClient::new(IO_DELAY).load(req.customer_id).await?;
    let subtotal = price_impl(&req);
    let total_cents = apply_rules_impl(subtotal, &customer);
    let items_count = items_count_impl(&req);

    let order = Order {
        customer: customer.clone(),
        total_cents,
        items_count,
    };
    OrderRepository::new(IO_DELAY).persist(&order).await?;

    Ok(Response {
        customer_name: customer.name,
        total_cents,
        items_count,
    })
}

use demo::{handle, Item, PipelineError, Request, Response};

#[tokio::test]
async fn handle_calcula_o_total_do_pedido() {
    let req = Request {
        customer_id: 1,
        items: vec![
            Item { unit_price_cents: 1000, quantity: 2 },
            Item { unit_price_cents: 500, quantity: 1 },
        ],
    };

    let resp = handle(req).await.unwrap();

    assert_eq!(
        resp,
        Response {
            customer_name: "Cliente 1".to_string(),
            total_cents: 2500,
            items_count: 3,
        }
    );
}

#[tokio::test]
async fn erro_do_cliente_de_io_propaga() {
    let req = Request {
        customer_id: 404,
        items: vec![Item { unit_price_cents: 100, quantity: 1 }],
    };

    assert_eq!(handle(req).await, Err(PipelineError::CustomerNotFound));
}

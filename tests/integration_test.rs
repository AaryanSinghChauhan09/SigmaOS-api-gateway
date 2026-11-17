use sigmaos_api_gateway::{ApiGateway, Request, Response};

#[test]
fn test_basic_gateway_aaa() {
    // Arrange
    let mut gw = ApiGateway::new();
    gw.add_route("/test", |_| Ok(Response::ok()));

    // Act
    let req = Request::new("GET", "/test");
    let res = gw.handle(req);

    // Assert
    assert!(res.is_ok());
    assert_eq!(res.unwrap().status, 200);
}

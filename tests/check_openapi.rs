use openapi_to_rust::check_openapi;

#[test]
fn test_check_openapi_success() {
    #[check_openapi(schema = "./test-resources/test_schema.yaml")]
    enum TestEnum {
        #[openapi(value = "foo")]
        Foo,
        #[openapi(value = "bar")]
        Bar,
    }
}

use crate::compras::calcular_lista_compras;

pub mod db;
pub mod compras;
pub mod clientes;
pub mod produtos;
pub mod estoque;

#[tokio::main]
async fn main() {
    println!("aguarde... carregando banco de dados");
    let pool = db::setup_database().await.expect("Erro ao configurar o banco de dados");
    
    calcular_lista_compras(&pool).await.expect("Erro ao reunir pedidos");    

    println!("mostrar lista de compras do sistema");

    db::lista_compras(&pool).await.expect("Erro ao executar a consulta lista de compras");

    println!("");
    println!("");
    println!("produtos do sistema");

    produtos::lista_produtos(&pool).await.expect("Erro ao executar a consulta lista de compras");
}

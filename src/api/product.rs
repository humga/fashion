use poem::{error::BadRequest, handler, web::{Data, Json, Path},Result};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug,Serialize, Deserialize,sqlx::FromRow)]
pub struct ProductAttr{
    pub name: String,
    pub value: Vec<String>,
}

#[derive(Debug,Serialize ,Deserialize, sqlx::FromRow)]
pub struct ProductInfo {
    pub product_name: String,
    pub product_id: i32,
    pub pic: String,
    pub category: String,
    pub rating: f32,
    pub attr_title: String,
    pub attr: Vec<String>,
    pub price: f32,
}


#[derive(Debug,Serialize,Deserialize,sqlx::FromRow)]
struct Category {
    id: i32,
    name: String,
    level: i32,
    parent_id: i32,
}

#[derive(Debug,Serialize,sqlx::FromRow)]
struct Detail {
    t: String,
    title: String,
    detail: String,
}

#[derive(Debug,Serialize,sqlx::FromRow)]
struct Picture {
    t: i32,
    sort: i32,
    url: String,
}
#[derive(Debug,Serialize)]
struct ProductDetail {
    info: ProductInfo,
    pics: Vec<Picture>,
    details: Vec<Detail>,
}

#[derive(Debug,Serialize, sqlx::FromRow)]
pub struct Collection {
    id: i32,
    name: String,
    pic: String,
    sort: i32,
    // products: Vec<ProductInfo>,
}

#[derive(Debug,Serialize, sqlx::FromRow)]
pub struct Collections {
    id: i32,
    name: String,
    pic: String,
}

#[handler]
pub async fn get_categorys(state:Data<&PgPool>) -> Result<Json<Vec<Category>>>{
    let rows = sqlx::query_as::<_,Category>("select id,parent_id,name,level,nav_status from product_category where status = true")
                                                .fetch_all(state.0)
                                                .await.map_err(BadRequest)?;
 
    Ok(Json(rows))
}

#[handler]
pub async fn get_collections(state:Data<&PgPool>,Path(id):Path<i32>) -> Result<Json<Collections>>{
    let rows = sqlx::query_as::<_,Collections>("select id, name,pic from collection where status = true and level = 2 and id = ?")
                    .bind(id)
                    .fetch_one(state.0)
                    .await.map_err(BadRequest)?;
    Ok(Json(rows))
}

#[handler]
pub async fn get_collcetion(state:Data<&PgPool>,Path(id):Path<i32>) -> Result<Json<Collection>>{
    let rows = sqlx::query_as::<_,Collection>("select id, name,pic from collection where status = true and level = 1 and id = ?")
                            .bind(id)
                            .fetch_one(state.0)
                            .await.map_err(BadRequest)?;
    Ok(Json(rows))
}

#[handler]
pub async fn get_product_by_category(state:Data<&PgPool>,req:Json<Category>) -> Result<Json<Vec<ProductInfo>>>{

    let rows = sqlx::query_as::<_,ProductInfo>("SELECT b.ID,b.NAME,b.preview_pic,b.product_category_id,b.product_category_name,b.rating,b.price  from product_category a LEFT JOIN product b on b.product_category_id = a.id where b.product_category_name=?")
                                    .bind(req.name.clone())
                                    .fetch_all(state.0)
                                    .await.map_err(BadRequest)?;
    Ok(Json(rows))
}

#[handler]
pub async fn get_product_detail(state:Data<&PgPool>,Path(id):Path<i32>) -> Result<Json<ProductDetail>>{
    
    let info = sqlx::query_as::<_,ProductInfo>("SELECT ID,NAME,preview_pic,product_category_id,product_category_name,rating,price  from product_category where id=?")
                            .bind(id)
                            .fetch_one(state.0)
                            .await.map_err(BadRequest)?;
    let pics = sqlx::query_as::<_,Picture>("sql")
                            .bind(id)
                            .fetch_all(state.0)
                            .await.map_err(BadRequest)?;
    let details = sqlx::query_as::<_,Detail>("sql")
                            .bind(id)
                            .fetch_all(state.0)
                            .await.map_err(BadRequest)?;

     Ok(Json(ProductDetail{ info, pics, details })) 

}


pub async fn search_product(){
    
}
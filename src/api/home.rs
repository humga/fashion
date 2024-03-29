use poem::{
    error::BadRequest,
    handler,
    web::{Data, Json},
    Result,
};
use serde::Deserialize;
use sqlx::{self, PgPool, Row};

use super::product::{Collection, Collections, ProductInfo};

#[derive(Debug,Deserialize)]
struct User {
    pub id: i32,
    pub name: String,
}

#[handler]
pub async fn home_new_collection(state: Data<&PgPool>) -> Result<Json<Vec<Collection>>> {
    let rows = sqlx::query_as::<_,Collection>("select id,name,pic from collection where status = true and recommend_status = true and level=1 order by create_time desc limit 3")
                                                    .fetch_all(state.0).await.map_err(BadRequest)?;
    Ok(Json(rows))
}

#[handler]
pub async fn get_collections(state: Data<&PgPool>) -> Result<Json<Vec<Collections>>> {
    let rows = sqlx::query_as::<_,Collections>("select id,name,pic from collection where status = true and recommend_status = true and level=2 order by create_time desc limit 3")
                                                    .fetch_all(state.0).await.map_err(BadRequest)?;
    Ok(Json(rows))
}

#[handler]
pub async fn home_new_product(state: Data<&PgPool>) -> Result<Json<Vec<ProductInfo>>> {
    let rows = sqlx::query("SELECT ID,NAME,preview_pic,product_category_name,price,rating FROM product WHERE new_status=TRUE ORDER BY sort LIMIT 4")
            .fetch_all(state.0)
            .await
            .map_err(BadRequest)?
            .iter()
            .map(|row| ProductInfo{ 
                product_name: row.get("name"), 
                product_id: row.get("id"), 
                pic: row.get("preview_pic"), 
                category: row.get("product_category_name"), 
                rating: row.get("rating"), 
                attr_title:String::from("size"),
                attr: Vec::new(),
                price: row.get("price") 
            })
            .collect();
    Ok(Json(rows)) 
}

#[handler]
pub async fn home_recommend(req: Json<User>,state: Data<&PgPool>) -> Result<Json<Vec<ProductInfo>> >{
    let rows = sqlx::query("SELECT A.product_name AS NAME,A.product_id AS ID,b.preview_pic AS pic,b.product_category_name AS category,b.rating,b.price FROM product_recommend A INNER JOIN product b ON A.product_id=b.ID ORDER BY A.sort DESC limit 3;")
    .bind(req.id)
    .bind(req.name.clone())
    .fetch_all(state.0)
    .await
    .map_err(BadRequest)?
    .iter()
    .map(|row| ProductInfo{ 
        product_name: row.get("name"), 
        product_id: row.get("id"), 
        pic: row.get("pic"), 
        category: row.get("category"), 
        rating: row.get("rating"), 
        attr_title:String::from("size"),
        attr: Vec::new(),
        price: row.get("price") 
    })
    .collect();
Ok(Json(rows)) 
}

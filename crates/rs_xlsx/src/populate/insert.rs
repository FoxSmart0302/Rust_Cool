use crate::error::XResult;
use crate::populate::populate_columns::get_columns_list;
use crate::populate::INSERT_BATCH_SIZE;
use crate::product::Product;
use sqlx::{Pool, Postgres};

pub struct InsertParams<'a> {
    pub table_name: &'a str,
    pub cols: String,
    pub types: String,
    pub pg: Pool<Postgres>,
}

impl<'a> InsertParams<'a> {
    pub fn new(pg: Pool<Postgres>, table_name: &'a str) -> Self {
        let cols = get_columns_list()
            .into_iter()
            // .take(take_count)
            .map(|x| x.0)
            .collect::<Vec<&'static str>>()
            .join(", ")
            .trim_end_matches(", ")
            .to_string();

        let types = get_columns_list()
            .into_iter()
            // .take(take_count)
            .enumerate()
            .map(|(i, x)| format!("${}::{}[]", i + 1, x.1.to_string()))
            .collect::<Vec<String>>()
            .join(", ")
            .trim_end_matches(", ")
            .to_string();

        Self {
            types,
            cols,
            table_name,
            pg,
        }
    }
}

pub async fn insert<'a>(
    params: &InsertParams<'a>,
    products: impl Iterator<Item = &Product>,
    count: &mut usize,
) -> XResult<()> {
    let mut b1 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b2 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b3 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b4 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b5 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b6 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b7 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b8 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b9 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b10 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b11 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b12 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b13 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b14 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b15 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b16 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b17 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b18 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b19 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b20 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b21 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b22 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b23 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b24 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b25 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b26 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b27 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b28 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b29 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b30 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b31 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b32 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b33 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b34 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b35 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b36 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b37 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b38 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b39 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b40 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b41 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b42 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b43 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b44 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b45 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b46 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b47 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b48 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b49 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b50 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b51 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b52 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b53 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b54 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b55 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b56 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b57 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b58 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b59 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b60 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b61 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b62 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b63 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b64 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b65 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b66 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b67 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b68 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b69 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b70 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b71 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b72 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b73 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b74 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b75 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b76 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b77 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b78 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b79 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b80 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b81 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b82 = Vec::with_capacity(INSERT_BATCH_SIZE);
    let mut b83 = Vec::with_capacity(INSERT_BATCH_SIZE);

    let table_name = params.table_name;
    let cols = &params.cols;
    let types = &params.types;
    let base = format!("INSERT INTO \"{table_name}\" ({cols}) SELECT * FROM UNNEST({types})");

    for product in products {
        *count += 1;
        b1.push(product.id);
        b2.push(serde_json::to_string(&product.flags.0)?);
        b3.push(serde_json::to_string(&product.errors.0)?);
        b4.push(product.inputs.identifier.clone());
        b5.push(product.inputs.cost);
        b6.push(product.inputs.stock);
        b7.push(product.inputs.map);
        b8.push(product.inputs.supplier_title.clone());
        b9.push(product.inputs.supplier_sku.clone());
        b10.push(product.inputs.supplier_image.clone());
        b11.push(product.inputs.supplier_pack_quantity);
        b12.push(product.inputs.discount_per_product);
        b13.push(product.inputs.discount_supplier);
        b14.push(product.inputs.discount_cost);
        b15.push(product.inputs.total_cogs);
        b16.push(serde_json::to_string(&product.inputs.custom_columns)?);
        b17.push(product.asin.clone());
        b18.push(serde_json::to_string(&product.offers)?);
        b19.push(product.offers.total_offers_count);
        b20.push(product.offers.list_price);
        b21.push(serde_json::to_string(&product.images)?);
        b22.push(product.amazon_title.clone());
        b23.push(product.is_top_level_category);
        b24.push(product.category_raw.clone());
        b25.push(product.category.clone());
        b26.push(product.rank);
        b27.push(product.buybox_price);
        b28.push(product.amazon_pack_quantity);
        b29.push(product.number_of_variations);
        b30.push(serde_json::to_string(&product.variations_list)?);
        b31.push(product.parent_asin.clone());
        b32.push(serde_json::to_string(&product.sales_ranks)?);
        b33.push(product.dimensions.package_dimensions.length);
        b34.push(product.dimensions.package_dimensions.width);
        b35.push(product.dimensions.package_dimensions.height);
        b36.push(product.dimensions.package_dimensions.weight);
        b37.push(product.dimensions.package_dimensions.length_unit.clone());
        b38.push(product.dimensions.package_dimensions.width_unit.clone());
        b39.push(product.dimensions.package_dimensions.height_unit.clone());
        b40.push(product.dimensions.package_dimensions.weight_unit.clone());
        b41.push(product.dimensions.item_dimensions.length);
        b42.push(product.dimensions.item_dimensions.width);
        b43.push(product.dimensions.item_dimensions.height);
        b44.push(product.dimensions.item_dimensions.weight);
        b45.push(product.dimensions.item_dimensions.length_unit.clone());
        b46.push(product.dimensions.item_dimensions.width_unit.clone());
        b47.push(product.dimensions.item_dimensions.height_unit.clone());
        b48.push(product.dimensions.item_dimensions.weight_unit.clone());
        b49.push(product.amazon_fees.per_item_fee);
        b50.push(product.amazon_fees.fba_fees);
        b51.push(product.amazon_fees.variable_closing_fee);
        b52.push(product.amazon_fees.referral_fee);
        b53.push(product.amazon_fees.error.clone());
        b54.push(product.competitive_sellers);
        b55.push(product.brand.clone());
        b56.push(product.color.clone());
        b57.push(product.size_name.clone());
        b58.push(serde_json::to_string(&product.listing_restrictions)?);
        b59.push(product.financials.inbound_shipping);
        b60.push(product.financials.prep_cost);
        b61.push(product.financials.fba_storage_fees);
        b62.push(product.financials.net_revenue);
        b63.push(product.financials.profit);
        b64.push(product.financials.net_revenue);
        b65.push(product.financials.margin);
        b66.push(product.financials.roi);
        b67.push(product.size_tier.clone());
        b68.push(product.lowest_price_new_fba);
        b69.push(product.lowest_price_used_fba);
        b70.push(product.lowest_price_new_fbm);
        b71.push(product.lowest_price_used_fbm);
        b72.push(product.buybox_price_new);
        b73.push(product.buybox_price_used);
        b74.push(product.total_offers_count);
        b75.push(product.is_brand_blocklisted);
        b76.push(product.new_fba_offers_count);
        b77.push(product.new_fbm_offers_count);
        b78.push(product.is_adult);
        b79.push(product.is_hazmat);
        b80.push(product.is_meltable);
        b81.push(product.small_and_light_eligible);
        b82.push(product.small_and_light_eligible_reasons.as_i16());
        b83.push(product.bsr_percentage);
    }

    // println!("vals: {:#?}", b14);

    let q = sqlx::query(&base)
        .bind(b1)
        .bind(b2)
        .bind(b3)
        .bind(b4)
        .bind(b5)
        .bind(b6)
        .bind(b7)
        .bind(b8)
        .bind(b9)
        .bind(b10)
        .bind(b11)
        .bind(b12)
        .bind(b13)
        .bind(b14)
        .bind(b15)
        .bind(b16)
        .bind(b17)
        .bind(b18)
        .bind(b19)
        .bind(b20)
        .bind(b21)
        .bind(b22)
        .bind(b23)
        .bind(b24)
        .bind(b25)
        .bind(b26)
        .bind(b27)
        .bind(b28)
        .bind(b29)
        .bind(b30)
        .bind(b31)
        .bind(b32)
        .bind(b33)
        .bind(b34)
        .bind(b35)
        .bind(b36)
        .bind(b37)
        .bind(b38)
        .bind(b39)
        .bind(b40)
        .bind(b41)
        .bind(b42)
        .bind(b43)
        .bind(b44)
        .bind(b45)
        .bind(b46)
        .bind(b47)
        .bind(b48)
        .bind(b49)
        .bind(b50)
        .bind(b51)
        .bind(b52)
        .bind(b53)
        .bind(b54)
        .bind(b55)
        .bind(b56)
        .bind(b57)
        .bind(b58)
        .bind(b59)
        .bind(b60)
        .bind(b61)
        .bind(b62)
        .bind(b63)
        .bind(b64)
        .bind(b65)
        .bind(b66)
        .bind(b67)
        .bind(b68)
        .bind(b69)
        .bind(b70)
        .bind(b71)
        .bind(b72)
        .bind(b73)
        .bind(b74)
        .bind(b75)
        .bind(b76)
        .bind(b77)
        .bind(b78)
        .bind(b79)
        .bind(b80)
        .bind(b81)
        .bind(b82)
        .bind(b83);

    // println!("sql: {}", q.sql());
    q.execute(&params.pg).await?;

    Ok(())
}

// pub async fn insert_via_copy<'a>(
//     params: &InsertParams<'a>,
//     chunk: impl Iterator<Item = &Product>,
//     mut count: &mut usize,
// ) -> XResult<()> {
//     let mut r = params.pg.acquire().await?;
//
//     //
//     let mut buf = vec![];
//     let mut wtr = csv::Writer::from_writer(&mut buf);
//
//     // When writing records with Serde using structs, the header row is written
//     // automatically.
//     for product in chunk {
//         *count += 1;
//         wtr.serialize(product).unwrap();
//     }
//     // wtr.serialize(Record {
//     //     city: "Southborough".to_string(),
//     //     region: "MA".to_string(),
//     //     country: "United States".to_string(),
//     //     population: Some(9686),
//     // })?;
//     // wtr.serialize(Record {
//     //     city: "Northbridge".to_string(),
//     //     region: "MA".to_string(),
//     //     country: "United States".to_string(),
//     //     population: Some(14061),
//     // })?;
//     wtr.flush()?;
//     let buf = wtr.into_inner().unwrap().to_owned();
//
//     let s = String::from_utf8(buf)?;
//     println!("csv: {}", s);
//
//     let first = format!("COPY {} ({}) FROM STDIN", params.table_name, params.cols);
//     let mut t = r.copy_in_raw(&first).await?;
//     t.send("test".as_bytes()).await?;
//     // t.send("test".into()).await?;
//     t.finish().await?;
//
//     Ok(())
// }

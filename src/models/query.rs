//! Query Representation

#[derive(Clone)]
pub struct Query {
    pub select_v: Vec<String>,
    pub from_v: Vec<String>,
    pub where_v: Vec<String>,
    pub join_v: Vec<String>,
    pub order_v: Vec<String>,

    pub limit: i64,
    pub offset: i64,
    pub format: String,
    pub order_by: Vec<String>,
}

impl Query {
    pub fn format(&self) -> String {
        let select_string = self.select_v.join("\n,");
        let from_string = self.from_v.join("\n,");
        let where_string = self.where_v.join("\n,");
        let join_string = self.join_v.join("\n");
        let order_string = self.order_v.join("\n,");
        let limit_string = format!("LIMIT {} OFFSET {}",
            self.limit,
            self.offset
        );

        return rt_format!(self.format,
            select_string,
            from_string,
            where_string,
            join_string,
            order_string,
            limit_string
        ).unwrap()
    }
}
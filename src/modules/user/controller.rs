#[post("/dispenser", format = "application/json", data = "<dispenser>")]
async fn dispenser(bar : &State<Bar>,  dispenser: Json <DispenserDTO> ) -> status::Custom<Option<Json<DispenserDTO>>> {
    let mut db_obj: Dispenser = Dispenser{ jwt_secret: dispenser.jwt_secret.to_owned(), flow_volume: dispenser.flow_volume, _id: None};
    let new_id = bar.add_dispenser(&db_obj).await;
    match new_id{
        Some(id) => {
            db_obj._id = Some(id);

            let new_dispenser = DispenserDTO {
                id: Some(id.to_string()),
                jwt_secret : dispenser.jwt_secret.to_owned(),
                flow_volume: dispenser.flow_volume,
            };
            status::Custom(Status::Ok,Some(Json::from(new_dispenser)))
        },
        None => status::Custom(Status::InternalServerError, None)
    }
}

#[put("/dispenser/<id>/status", format = "application/json", data = "<tab>")]
async fn tab(bar: &State<Bar>, tab : Json<TabDTO>, id : String, dispenser :Dispenser) -> status::Custom<Option<Json<TabDTO>>> {
    if tab.status.eq("open"){
        let status = bar.open_tab(ObjectId::parse_str(String::from(id)).unwrap() , tab.updated_at.to_owned()).await;
        match status {
            BarResponse::TabHasBeenCreated => return status::Custom(Status::Accepted,Some(Json::from(tab))),
            BarResponse::DispenserIsOpen => return status::Custom(Status::Conflict,Some(Json::from(tab))),
            BarResponse::DispenserNotFound => return status::Custom(Status::NotFound,None),
            _ => return status::Custom(Status::InternalServerError,Some(Json::from(tab))),
        }
    }
    else if tab.status.eq("close"){
        let status = bar.close_tab(ObjectId::parse_str(String::from(id)).unwrap() , tab.updated_at.to_owned()).await;
        match status {
            BarResponse::TabHasBeenUpdated => return status::Custom(Status::Accepted,Some(Json::from(tab))),
            BarResponse::DispenserIsClosed => return status::Custom(Status::Conflict,Some(Json::from(tab))),
            BarResponse::DispenserNotFound => return status::Custom(Status::NotFound,None),
            _ => return status::Custom(Status::InternalServerError,Some(Json::from(tab))),
        }
    } else{
        //If status is neither open nor closed..... return bad request
        return status::Custom(Status::BadRequest,Some(Json::from(tab)));
    }
} 

#[get("/dispenser/<id>/spending")]
async fn spending(bar: &State<Bar>, id : String) -> status::Custom<Option<Json<SpendingDTO>>> {
    let spending = bar.get_spending(ObjectId::parse_str(String::from(id)).unwrap()).await;
    if spending.is_none(){
        return status::Custom(Status::NotFound,None);
    }
    else{
        return status::Custom(Status::Ok,Some(Json::from(spending.unwrap())));
    }
}
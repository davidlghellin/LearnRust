use crate::models::pokemon_name::PokemonName;
use crate::models::pokemon_number::PokemonNumber;
use crate::models::pokemon_type::PokemonTypes;
use crate::models::reques::Request;
use crate::models::response::Response;

/*
fn execute(req: Request) -> u16 {
    req.number
}

fn execute(req: Request) -> Response {
    Response::BadRequest
}
*/
fn execute(req: Request) -> Response {
    match (
        PokemonNumber::try_from(req.number),
        PokemonName::try_from(req.name),
        PokemonTypes::try_from(req.types),
    ) {
        (Ok(number), Ok(_), Ok(_)) => Response::Ok(u16::from(number)),
        _ => Response::BadRequest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(req);

        match res {
            Response::Ok(res_number) => assert_eq!(res_number, number),
            _ => unreachable!(),
        };
    }
}

#[test]
fn it_should_return_a_bad_request_error_when_request_is_invalid() {
    let req = Request {
        number: 25,
        name: String::from(""),
        types: vec![String::from("Electric")],
    };

    let res = execute(req);

    match res {
        Response::BadRequest => {}
        _ => unreachable!(),
    };
}

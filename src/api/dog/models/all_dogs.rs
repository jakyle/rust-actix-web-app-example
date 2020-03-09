use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllDogs {
    pub message: Message,
    pub status: String,
}

impl Responder for AllDogs {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub affenpinscher: Vec<String>,
    pub african: Vec<String>,
    pub airedale: Vec<String>,
    pub akita: Vec<String>,
    pub appenzeller: Vec<String>,
    pub australian: Vec<String>,
    pub basenji: Vec<String>,
    pub beagle: Vec<String>,
    pub bluetick: Vec<String>,
    pub borzoi: Vec<String>,
    pub bouvier: Vec<String>,
    pub boxer: Vec<String>,
    pub brabancon: Vec<String>,
    pub briard: Vec<String>,
    pub buhund: Vec<String>,
    pub bulldog: Vec<String>,
    pub bullterrier: Vec<String>,
    pub cairn: Vec<String>,
    pub cattledog: Vec<String>,
    pub chihuahua: Vec<String>,
    pub chow: Vec<String>,
    pub clumber: Vec<String>,
    pub cockapoo: Vec<String>,
    pub collie: Vec<String>,
    pub coonhound: Vec<String>,
    pub corgi: Vec<String>,
    pub cotondetulear: Vec<String>,
    pub dachshund: Vec<String>,
    pub dalmatian: Vec<String>,
    pub dane: Vec<String>,
    pub deerhound: Vec<String>,
    pub dhole: Vec<String>,
    pub dingo: Vec<String>,
    pub doberman: Vec<String>,
    pub elkhound: Vec<String>,
    pub entlebucher: Vec<String>,
    pub eskimo: Vec<String>,
    pub frise: Vec<String>,
    pub germanshepherd: Vec<String>,
    pub greyhound: Vec<String>,
    pub groenendael: Vec<String>,
    pub havanese: Vec<String>,
    pub hound: Vec<String>,
    pub husky: Vec<String>,
    pub keeshond: Vec<String>,
    pub kelpie: Vec<String>,
    pub komondor: Vec<String>,
    pub kuvasz: Vec<String>,
    pub labrador: Vec<String>,
    pub leonberg: Vec<String>,
    pub lhasa: Vec<String>,
    pub malamute: Vec<String>,
    pub malinois: Vec<String>,
    pub maltese: Vec<String>,
    pub mastiff: Vec<String>,
    pub mexicanhairless: Vec<String>,
    pub mix: Vec<String>,
    pub mountain: Vec<String>,
    pub newfoundland: Vec<String>,
    pub otterhound: Vec<String>,
    pub papillon: Vec<String>,
    pub pekinese: Vec<String>,
    pub pembroke: Vec<String>,
    pub pinscher: Vec<String>,
    pub pitbull: Vec<String>,
    pub pointer: Vec<String>,
    pub pomeranian: Vec<String>,
    pub poodle: Vec<String>,
    pub pug: Vec<String>,
    pub puggle: Vec<String>,
    pub pyrenees: Vec<String>,
    pub redbone: Vec<String>,
    pub retriever: Vec<String>,
    pub ridgeback: Vec<String>,
    pub rottweiler: Vec<String>,
    pub saluki: Vec<String>,
    pub samoyed: Vec<String>,
    pub schipperke: Vec<String>,
    pub schnauzer: Vec<String>,
    pub setter: Vec<String>,
    pub sheepdog: Vec<String>,
    pub shiba: Vec<String>,
    pub shihtzu: Vec<String>,
    pub spaniel: Vec<String>,
    pub springer: Vec<String>,
    pub stbernard: Vec<String>,
    pub terrier: Vec<String>,
    pub vizsla: Vec<String>,
    pub waterdog: Vec<String>,
    pub weimaraner: Vec<String>,
    pub whippet: Vec<String>,
    pub wolfhound: Vec<String>,
}

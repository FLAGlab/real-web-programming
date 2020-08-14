use yew::prelude::*;

use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};

use yew::events::KeyboardEvent;

use yew::format::{Nothing, Text};

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

/// Topics page
pub struct Topics;

impl Component for Topics {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Topics {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
                <div class="container">
                    <div class="app">
                        <header class="app-header">
                            <div class="mb-4">
                                <h1>{ "Topics" }</h1>
                            </div>
                            <div class="row">
                                <div class="col-md-6">
                                    <div class="list-group">
                                    <a href="#" class="list-group-item list-group-item-action">{"Cardozo, Nicolas"}</a>
                                    <a href="#" class="list-group-item list-group-item-action">{"Mariño, Olga"}</a>
                                    <a href="#" class="list-group-item list-group-item-action">{"Castro Barrera, Harold"}</a>
                                    <a href="#" class="list-group-item list-group-item-action">{"Correal Torres, Dario Ernesto"}</a>
                                    <a href="#" class="list-group-item list-group-item-action">{"Duitama, Jorge"}</a>
                                    <a href="#" class="list-group-item list-group-item-action">{"Avila Cifuentes, Oscar Javier"}</a>
                                    <a href="#" class="list-group-item list-group-item-action">{"Romero Gutierres, German"}</a>
                                    <a href="#" class="list-group-item list-group-item-action">{"Linares Vásquez, Mario"}</a>
                                    <a href="#" class="list-group-item list-group-item-action">{"Takahashi Rodriguez, Silvia"}</a>
                                    <a href="#" class="list-group-item list-group-item-action">{"Villamil Giraldo, Maria del Pilar"}</a>
                                    <a href="#" class="list-group-item list-group-item-action">{"Donoso Meisel, Yezyd"}</a>
                                </div>
                            </div>
                            <div class="col-md-6">
                                <h3>{"Details"}</h3>
                                <form action="/">
                                    <div class="form-group">
                                        <label for="title">{"Title"}</label>
                                        <input class="form-control" type="text" id="title" name="title" disabled=true value="Real Web Programming"/>
                                    </div>
                                    <div class="form-group">
                                        <label for="description">{"Description"}</label>
                                        <textarea class="form-control" id="description" name="description" rows="16" disabled=true>
                                        {
        "Context
Information systems are used to manage processes and information in almost every organization. Such systems, as they manage sensitive and timely information of the organization, have high reliability requirements. Moreover, as organizations are more and more distributed, such systems are migrating to the web to enable remote and distributed access. However, web technology is not at par with reliable systems. Therefore, more often than not, this systems fail.

Project proposal
In this project we are going to implement a realiable information an process management system for academic organizations using cutting-edge technology to ensure the reliability of the system. The system must be able to incorporate appointment bookign and scheduling, course registry, and employment exchange, among others.

Implementation plan
The project should be implemented using cutting edge web technology –that is, use Rust [1] to be deployed in the web using WebAssembly [2]."
                                        }
                                        </textarea>
                                    </div>
                                    <input class="btn btn-primary" type="submit" value="Aplicar"/>
                                </form>
                            </div>
                        </div>
                    </header>
                </div>
            </div>
        }
    }
}

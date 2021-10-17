import * as React from "react";
import * as ReactDOM from "react-dom";
import { HashRouter, Route } from "react-router-dom";
import "./index.css";
import Layout from "./Components/Layout";

const App = () => (  
    <div>
      <HashRouter>
        <Route path="/" component={Layout} /> 
      </HashRouter>
    </div> 
);

ReactDOM.render(<App />, document.querySelector("#root"));

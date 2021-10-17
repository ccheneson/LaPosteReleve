import React from "react";
import { Route, HashRouter } from "react-router-dom";
import Activities from "./Activities";
import Graph from "./Graph";
import Header from "./Header";
import TagPatternList from "./TagPatternList";

const Layout = () => {
    return(
        <div>
            <Header />
          <div>
            <Route exact path="/" component={Activities} />    
            <Route path="/stats/per_month" component={Graph} />    
            <Route path="/tags/pattern" component={TagPatternList} /> 
          </div>
        </div>
    )
}

export default Layout;
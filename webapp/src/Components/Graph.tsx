import { Bar } from "react-chartjs-2";
import { monthByIndex } from "./Activities";
import React, { useState, useEffect } from "react";
import BarStats from "./BarStats";
import index from "../index.css";
 

const Graph = () => {

    return (
        <div className={index.statContainerRoot}>                                    
            <div className={index.statContainer}>
                <BarStats data_source="http://localhost:3030/api/stats/per_month/tag?value=FREEMOBILE" />
            </div>
            <div className={index.statContainer}>
                <BarStats data_source="http://localhost:3030/api/stats/per_month/tag?value=RETRAIT" />
            </div>
            <div className={index.statContainer}>
                <BarStats data_source="http://localhost:3030/api/stats/per_month/tag?value=EDF" />
            </div>
            <div className={index.statContainer}>
                <BarStats data_source="http://localhost:3030/api/stats/per_month/tag?value=PARIS" />
            </div>         
      </div>
    )
};

export default Graph;
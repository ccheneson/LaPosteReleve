import React, { useState, useEffect } from "react";
import BarStats from "./BarStats";
import index from "../index.css";
 
 
type StatsDataJson = {
    amount: number,
    month: number
}

type StatsJson = {
    tags: string[],
    data: StatsDataJson[]
}


const Graph = () => {

    const [statsFreeMobile, setStatsFreeMobile] = useState<StatsJson | undefined>(undefined)
    const [statsRetrait, setStatsRetrait] = useState<StatsJson | undefined>(undefined)
    const [statsEdf, setStatsEdf] = useState<StatsJson | undefined>(undefined)
    const [statsParis, setStatsParis] = useState<StatsJson | undefined>(undefined)


    useEffect(() => {
        fetch("http://localhost:3030/api/stats/per_month/tag?value=FREEMOBILE", { mode: 'cors'})
            .then(response => response.json())
            .then(data => setStatsFreeMobile(data))
    }, [])

    useEffect(() => {
        fetch("http://localhost:3030/api/stats/per_month/tag?value=RETRAIT", { mode: 'cors'})
            .then(response => response.json())
            .then(data => setStatsRetrait(data))
    }, [])

    useEffect(() => {
        fetch("http://localhost:3030/api/stats/per_month/tag?value=EDF", { mode: 'cors'})
            .then(response => response.json())
            .then(data => setStatsEdf(data))
    }, [])

    useEffect(() => {
        fetch("http://localhost:3030/api/stats/per_month/tag?value=PARIS", { mode: 'cors'})
            .then(response => response.json())
            .then(data => setStatsParis(data))
    }, [])

        


    return (        
        <div className={index.statContainerRoot}>
            <div className={index.statContainer}>
                <BarStats dataJson={statsFreeMobile} />
            </div>
            <div className={index.statContainer}>
                <BarStats dataJson={statsRetrait} />
            </div>
            <div className={index.statContainer}>
                <BarStats dataJson={statsEdf} />
            </div>
            <div className={index.statContainer}>
                <BarStats dataJson={statsParis} />
            </div>         
      </div>
    )
};

export default Graph;
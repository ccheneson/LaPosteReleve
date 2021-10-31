import { Bar } from "react-chartjs-2";
import React, { useState, useEffect } from "react";
import { monthByIndex } from "./ActivitiesTable";

 
type StatsDataJson = {
    amount: number,
    month: number
}

type StatsJson = {
    tags: string[],
    data: StatsDataJson[]
}

type StatsProps = {
    dataJson: StatsJson
}

const BarStats = (props: StatsProps) => {
 
    const [stats, setStats] = useState<StatsJson>(undefined)

    useEffect(() => {
        setStats(props.dataJson)
    })
    
    return (
        <div>
        { stats 
        ?
        <Bar
            data={{
                labels: stats?.data.map(m => monthByIndex[m.month - 1]),
                datasets: [{
                    label: `Depense en € pour tag ${stats?.tags.join(', ')}`,
                    data: stats?.data.map(m => m.amount)
                }]
            }}
            width={ 300 }
            height={ 400 }        
            options= {{ 
                maintainAspectRatio: false,
                indexAxis : 'x'
            }}
        />
        :
            <h3>Error loading page: can not reach data source</h3>
        }
      </div>
    )
};

export default BarStats;
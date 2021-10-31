import React, { useState, useEffect, useRef } from "react";
import ActivitiesTable from "./ActivitiesTable";


type StatsJson = {
    amount_plus: number,
    amount_minus: number
}
type ActivitiesPerMonthJson = {
    month_index : number,
    stats : StatsJson,
    activities: ActivityDetailsJson[] 
}

type ActivityDetailsJson = {
    row_id: number,
    date: string,
    statement: string,
    amount: number,
    tag_pattern_id: number | null
};

type BalanceJson = {
    date: string,
    amount: number
};

type TagsJson = {
    [tag_pattern_id: number] : string[]
};

const Activities = () => {

    const [activities, setActivities] = useState<ActivitiesPerMonthJson[]>([])
    const [balance, setBalance] = useState<BalanceJson>(undefined)
    const [tags, setTags] = useState<TagsJson>(undefined)

    useEffect(() => {
        fetch("http://localhost:3030/api/tags", { mode: 'cors'})
            .then(response => response.json())
            .then(data => setTags(data))
    }, [])

    useEffect(() => {        
        fetch("http://localhost:3030/api/activities", { mode: 'cors'})
            .then(response => response.json())
            .then(data => setActivities(data))
    }, [])

    useEffect(() => {        
        fetch("http://localhost:3030/api/balance", { mode: 'cors'})
            .then(response => response.json())
            .then(data => setBalance(data))
    }, [])
    
    return (      
        <div>
            { activities.length === 0 || balance == null || tags == null
            ?
            <div>
                <h3>Error loading page: can not reach data source</h3>
            </div>
            :
            <div>
                <ActivitiesTable activities={activities} balance={balance} tags={tags}/>
            </div>
            }
        </div >
    );
};

export default Activities;
export { ActivitiesPerMonthJson, ActivityDetailsJson };

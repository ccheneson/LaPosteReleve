import React, { useState, useEffect, useRef } from "react";
import ActivityDetails, { ActivityDetailsMonthly } from "./ActivityDetails"
import classNames from "classnames"
import index from "../index.css";
import Search from "./Search";

type CssByDate = {
    previousDate: string,
    style: string
}


const monthByIndex = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
]

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

    const [activities, setActivities] = useState<ActivitiesPerMonthJson[] | []>([])
    const [balance, setBalance] = useState<BalanceJson | undefined>(undefined)
    const [tags, setTags] = useState<TagsJson | undefined>(undefined)
    const activitiesJson = useRef([]);

    useEffect(() => {
        fetch("http://localhost:3030/api/tags", { mode: 'cors'})
            .then(response => response.json())
            .then(data => setTags(data))
    }, [])


    useEffect(() => {
        fetch("http://localhost:3030/api/activities", { mode: 'cors'})
            .then(response => response.json())
            .then(data => {
                activitiesJson.current = data;
                setActivities(data)
            })
    }, [])

    useEffect(() => {
        fetch("http://localhost:3030/api/balance", { mode: 'cors'})
            .then(response => response.json())
            .then(data => setBalance(data))
    }, [])

    const byMonthDesc = (a: number, b: number) => b - a

    var cssByDate: CssByDate = { previousDate: "", style: "" };

    const toggleRowStyle = (date: string) => {
        if (cssByDate === undefined || date !== cssByDate.previousDate) {
            cssByDate.style = cssByDate.style === index.colorRowG ? index.colorRowB : index.colorRowG;
            cssByDate.previousDate = date;
        }
        return cssByDate.style;
    }

    const formatDate = (d:string) => {
        const date = new Date(d)
        return date.getDate().toString().padStart(2,'0')   + "/" + (date.getMonth() + 1 ).toString().padStart(2,'0') + "/" + date.getFullYear()
    }

    const updateSearchResult = (activitiesJson: ActivitiesPerMonthJson[]) => (e: React.FormEvent<HTMLInputElement>) => {
        const searchPattern = e.currentTarget.value.toLowerCase();
        const checkPatternInTags = (tagPatternId: number|null, searchPattern: string) => {
            if (tagPatternId == null && searchPattern == "null") {
                return true;
            } else if (tagPatternId == null) {
                return false;
            } else if (tags && tagPatternId in tags) {
                return tags[tagPatternId].filter(e => e.toString().toLowerCase().includes(searchPattern)).length > 0;
            } else {
                return false;
            }
        }
        const updated = activitiesJson.map(e => {
            const newActivities = e.activities
            .filter(
                a => a.amount.toString().toLowerCase().includes(searchPattern) ||
                     a.date.toString().toLowerCase().includes(searchPattern) ||
                     a.statement.toLowerCase().includes(searchPattern) ||
                     checkPatternInTags(a.tag_pattern_id, searchPattern)
            )
            return { month_index : e.month_index, stats : e.stats , activities : newActivities}
        })
        setActivities(updated);
    }

    return (      
        <div>
            { activities.length === 0
            ?
            <div>
                <h3>Error loading page: can not reach data source</h3>
            </div>
            :
            <div>
                <Search onChange={updateSearchResult(activitiesJson.current)}/>
                <br />
                <table id="account" className={index.large_table}>
                    <tbody>
                        <tr>
                            <th>Month</th>
                            <th>Date</th>
                            <th>Libelle</th>
                            <th>Tags</th>
                            <th>Montant {balance ? "au " + formatDate(balance?.date) : ""}<br />
                            {`${balance?.amount} €`}
                            </th>
                        </tr>
                        {
                            activities.map((activitiesPerMonth: ActivitiesPerMonthJson) =>
                                activitiesPerMonth.activities.map((activity, i) =>
                                    (i === 0) ?
                                        <ActivityDetailsMonthly key={activity.row_id} className={toggleRowStyle(activity.date)}
                                            date={formatDate(activity.date)} statement={activity.statement}
                                            amount={activity.amount}
                                            statsPlus={`+${activitiesPerMonth.stats.amount_plus.toFixed(2)}`}
                                            statsMinus={`${activitiesPerMonth.stats.amount_minus.toFixed(2)}`}
                                            month={monthByIndex[activitiesPerMonth.month_index - 1]}
                                            tags={tags && activity.tag_pattern_id && activity.tag_pattern_id in tags ? tags[activity.tag_pattern_id] : []} />
                                        :
                                        <ActivityDetails key={activity.row_id} className={toggleRowStyle(activity.date)}
                                            date={formatDate(activity.date)} statement={activity.statement}
                                            amount={activity.amount} tags={tags && activity.tag_pattern_id && activity.tag_pattern_id in tags ? tags[activity.tag_pattern_id] : []} />
                                        )
                            )}
                    </tbody>
                </table>
            </div>
            }
        </div >
    );
};

export default Activities;
export { monthByIndex, ActivitiesPerMonthJson, ActivityDetailsJson };

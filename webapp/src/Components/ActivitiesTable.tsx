import React, { useState, useRef } from "react";
import ActivityDetails, { ActivityDetailsMonthly } from "./ActivityDetails"
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

type ActivitiesPerMonthJson = {
    month_index: number,
    stats: {
        amount_plus: number,
        amount_minus: number
    },
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
    [tag_pattern_id: number]: string[]
};

type ActivitiesTableProps = {
    activities: ActivitiesPerMonthJson[],
    balance: BalanceJson
    tags: TagsJson
}

const ActivitiesTable = (props: ActivitiesTableProps) => {

    let balance = props.balance;
    let tags = props.tags;

    const [activities, setActivities] = useState<ActivitiesPerMonthJson[]>(props.activities)
    const activitiesJson = useRef(props.activities);

    var cssByDate: CssByDate = { previousDate: "", style: "" };

    const toggleRowStyle = (date: string) => {
        if (cssByDate === undefined || date !== cssByDate.previousDate) {
            cssByDate.style = cssByDate.style === index.colorRowG ? index.colorRowB : index.colorRowG;
            cssByDate.previousDate = date;
        }
        return cssByDate.style;
    }

    const formatDate = (d: string) => {
        const date = new Date(d)
        return date.getDate().toString().padStart(2, '0') + "/" + (date.getMonth() + 1).toString().padStart(2, '0') + "/" + date.getFullYear()
    }

    const SEARCH_UNTAGGED = "null";
    const check_null_search = (pattern: string) => {
        if (pattern.length <= SEARCH_UNTAGGED.length && SEARCH_UNTAGGED.startsWith(pattern)) {
            return true;
        } else {
            return false;
        }
    }

    const updateSearchResult = (activitiesJson: ActivitiesPerMonthJson[]) => (e: React.FormEvent<HTMLInputElement>) => {
        const searchPattern = e.currentTarget.value.toLowerCase();
        const checkPatternInTags = (tagPatternId: number | null, searchPattern: string) => {
            if (tagPatternId == null && check_null_search(searchPattern)) {
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

            return { month_index: e.month_index, stats: e.stats, activities: newActivities }
        })
        setActivities(updated);
    }

    const checkIfResultEmpty = (activities: ActivitiesPerMonthJson[]) => {
        return activities.filter((e: ActivitiesPerMonthJson) => e.activities.length > 0).length == 0;
    }

    return (
        <div>
            {activities.length === 0
                ?
                <div>
                    <h3>Error while loading page: can not reach data source</h3>
                </div>
                :
                <div>
                    <Search onChange={updateSearchResult(activitiesJson.current)} />
                    <br />
                    {checkIfResultEmpty(activities)
                        ?
                        <div>
                            <h3>No result matching</h3>
                        </div>
                        :
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
                    }
                </div>
            }
        </div >
    );
};

export default ActivitiesTable;
export { monthByIndex, ActivitiesPerMonthJson, ActivityDetailsJson };

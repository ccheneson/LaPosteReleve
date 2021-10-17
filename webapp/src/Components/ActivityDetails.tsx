import React from "react";
import index from "../index.css";


type ActivityDetailsProps = {
    className: string,
    date: string,
    statement: string,
    amount: number,
    tags: string[]
};

type ActivityDetailsMonthlyProps = {
    month: string,
    statsPlus: string,
    statsMinus: string,
    className: string,
    date: string,
    statement: string,
    amount: number,
    tags: string[]
};

const ActivityDetails: React.FC<ActivityDetailsProps> = ( activity : ActivityDetailsProps) => {

    return (
        <tr className={activity.className}>
            <td></td>
            <td>{activity.date}</td>
            <td>{activity.statement}</td>
            <td>{activity.tags.join(', ')}</td>
            <td  className={ activity.amount >= 0 ? index.amountPlus : index.amountMinus }>{activity.amount.toFixed(2)}</td>
        </tr>
    );
};

const ActivityDetailsMonthly: React.FC<ActivityDetailsMonthlyProps> = ( activity : ActivityDetailsMonthlyProps) => {
    return (
        <tr className={activity.className}>
            <td className={index.month}>{activity.month}<br />{activity.statsPlus}<br/>{activity.statsMinus}</td>
            <td>{activity.date}</td>
            <td>{activity.statement}</td>
            <td>{activity.tags.join(', ')}</td>
            <td className={ activity.amount >= 0 ? index.amountPlus : index.amountMinus }>{activity.amount.toFixed(2)}</td>
        </tr>
    );
};
export default ActivityDetails;
export { ActivityDetailsMonthly };

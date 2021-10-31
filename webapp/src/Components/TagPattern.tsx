import React, { useState, useEffect } from "react";
import TagPatternList from "./TagPatternList";

type TagPatternJson = {
    pattern: string,
    tags: string[]
}


const TagPattern = () => {

    let [tagPatterns, setTagPatterns] = useState<TagPatternJson[]>(undefined)

    useEffect(() => {
        fetch("http://localhost:3030/api/tags/pattern", { mode: 'cors' })
            .then(response => response.json())
            .then(data => {
                setTagPatterns(data)
            })
    }, [])


    return (
        <div>
            {tagPatterns
                ?
                <TagPatternList patterns={tagPatterns} />
                :
                <h3>Error loading page: can not reach data source</h3>
            }
        </div>)
};

export default TagPattern;
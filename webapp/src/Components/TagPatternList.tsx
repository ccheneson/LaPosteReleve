import React, { useState, useEffect } from "react";

type TagPatternJson = {
    pattern: string,
    tags: string[]
}


const TagPatternList = () => {

    let [tagPatterns, setTagPatterns] = useState<TagPatternJson[] | undefined>(undefined)

    useEffect(() => {
        fetch("http://localhost:3030/api/tags/pattern", { mode: 'cors'})
            .then(response => response.json())
            .then(data => setTagPatterns(data))
    }, [])


    return (
        <div>
        {   tagPatterns 
        
        ?            
            <table>
                <tbody>
                    <tr>
                        <th>Pattern</th>
                        <th>tag</th>
                    </tr>
                    { tagPatterns.map(tp => 
                        <tr>
                            
                            <td>{ tp.pattern }</td>
                            <td>{ tp.tags.join(", ")}</td>
                        </tr>
                    )}
                </tbody>      
            </table>    
        :
            <h3>Error loading page: can not reach data source</h3>
        }
    </div>)
};

export default TagPatternList;
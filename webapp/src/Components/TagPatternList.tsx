import React, { useState, useEffect } from "react";

type TagPatternJson = {
    pattern: string,
    tags: string[]
}

type TagPatternProps = {
    patterns : TagPatternJson[]
}

const TagPatternList = (props: TagPatternProps) => {

    const [tagPatterns, setTagPatterns] = useState<TagPatternJson[]>(props.patterns)

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
                        <tr key={tp.pattern}>
                            
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
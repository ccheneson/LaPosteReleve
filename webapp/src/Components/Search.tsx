import React, { useState } from "react";


type SearchProps = {
    onChange : (value : React.FormEvent<HTMLInputElement>) => void
}

const Search = (props: SearchProps) => {

    return(
        <div>
            Search: <input type="text" onChange={props.onChange}/>
        </div>
    )
}

export default Search;
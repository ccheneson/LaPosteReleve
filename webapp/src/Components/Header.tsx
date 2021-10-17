import { Link } from 'react-router-dom';
import React from 'react';

const Header = () => {

    return (
        <div>
          <ul>
            <li><Link to="/">Activities</Link></li>
            <li><Link to="/stats/per_month">Stats Per Month</Link></li>
            <li><Link to="/tags/pattern">Tag Patterns</Link></li>
          </ul>
        </div>
    )

};

export default Header;
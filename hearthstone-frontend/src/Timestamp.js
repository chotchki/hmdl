import React, { useState, useEffect } from 'react';
import { DateTime } from 'luxon';

export function Timestamp(props) {
    const [lastSeen, setLastSeen] = useState(DateTime.fromISO(props.lastSeen, { zone: 'utc' }).toRelative());

    return (
        <span key={lastSeen}>
            {lastSeen}
        </span >
    );
}

export default Timestamp;
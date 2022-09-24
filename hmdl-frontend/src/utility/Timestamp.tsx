import React, { useState } from 'react';
import { DateTime } from 'luxon';
import PropTypes from 'prop-types';

export function Timestamp(props) {
  const [lastSeen] = useState(DateTime.fromISO(props.lastSeen, { zone: 'utc' }).toRelative());

  return (
    <span key={lastSeen}>
      {lastSeen}
    </span >
  );
}

Timestamp.propTypes = {
  lastSeen: PropTypes.string,
};

export default Timestamp;

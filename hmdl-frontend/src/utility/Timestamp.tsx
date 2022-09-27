import { useState } from 'react';
import { DateTime } from 'luxon';

type TimestampProps = {
  lastSeen: string
};

const Timestamp = ({ lastSeen }: TimestampProps): JSX.Element => {
  const [lastSeenRel] = useState(DateTime.fromISO(lastSeen, { zone: 'utc' }).toRelative());

  return (
    <span key={lastSeen}>
      {lastSeenRel}
    </span >
  );
}

export default Timestamp;

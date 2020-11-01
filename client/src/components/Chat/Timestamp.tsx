import React from 'react';


type DateProps = {
  timestamp: string;
};

const Timestamp: React.FC<DateProps> = (props) => {
  const parsed = new Date(Date.parse(props.timestamp));
  return (
    <span>
      {parsed.getHours().toString().padStart(2, '0')}
      :
      {parsed.getMinutes().toString().padStart(2, '0')}
      :
      {parsed.getSeconds().toString().padStart(2, '0')}
    </span>
  );
};

export default Timestamp;

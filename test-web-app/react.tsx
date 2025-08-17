import { useState } from "react";
import { Button, Space } from "antd";

const App: React.FC = () => {
  const [count, setCount] = useState(0);

  const handleClick = () => {
    setCount(count + 1);
  };

  return (
    <Space wrap>
      <Button type="primary">Primary Button</Button>
      <Button onClick={handleClick}>Default Button</Button>
    </Space>
  );
};

export default App;

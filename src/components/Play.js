import { Box, Button, RadioButtonGroup, Text, TextInput, Avatar } from 'grommet';
import React, { useState } from 'react';
import { Panel } from './Main';
import { utils } from 'near-api-js';

export const Play = ({playTarget, contract}) => {
  const [blends, setBlends] = useState({ b1: "", b2: "", b3: "" });
  const amount = 100;
  const handleClean = () => {
    setBlends({ b1: "", b2: "", b3: "" })
  }

  const handleSubmit = () => {
    contract.play_move({
      args: {"id": playTarget.id, "blend_op": [blends.b1, blends.b2, blends.b3]},
      amount: playTarget.amount
    })
  }
  return <Panel>
    <Text>Make a new move!</Text>
    <Text>Chooise a blend:</Text>
    <Box gap='small' border={{ color: 'c2' }} pad='small'>
      <RadioButtonGroup
        name='blend1'
        options={[
          { label: <Avatar src={'rock-icon-grey.png'} size='small' />, value: 'Gawi' },
          { label: <Avatar src={'paper-icon-grey.png'} size='small' />, value: 'Bawi' },
          { label: <Avatar src={'scissors-icon-grey.png'} size='small' />, value: 'Bo' },
        ]}
        value={blends.b1}
        onChange={(e) => setBlends({ ...blends, b1: e.target.value })}
        direction='row'
      />
      <RadioButtonGroup
        name='blend2'
        options={[
          { label: <Avatar src={'rock-icon-grey.png'} size='small' />, value: 'Gawi' },
          { label: <Avatar src={'paper-icon-grey.png'} size='small' />, value: 'Bawi' },
          { label: <Avatar src={'scissors-icon-grey.png'} size='small' />, value: 'Bo' },
        ]}
        value={blends.b2}
        onChange={(e) => setBlends({ ...blends, b2: e.target.value })}
        direction='row'
      />
      <RadioButtonGroup
        name='blend3'
        options={[
          { label: <Avatar src={'rock-icon-grey.png'} size='small' />, value: 'Gawi' },
          { label: <Avatar src={'paper-icon-grey.png'} size='small' />, value: 'Bawi' },
          { label: <Avatar src={'scissors-icon-grey.png'} size='small' />, value: 'Bo' },
        ]}
        value={blends.b3}
        onChange={(e) => setBlends({ ...blends, b3: e.target.value })}
        direction='row'
      />
    </Box>
    <Box direction='row' align='center' gap='small'>
      <Text margin={{ right: 'medium' }}>Amount</Text>
      <TextInput size='small' textAlign='end' value={utils.format.formatNearAmount(playTarget.amount)} disabled />
    </Box>
    <Box direction='row' alignSelf='end' gap='medium' pad={{ right: 'large' }}>
      <Button onClick={handleClean} label="clear" />
      <Button onClick={handleSubmit} disabled={amount > 0 && blends.b1 !== '' && blends.b2 !== '' && blends.b3 !== '' ? false : true} label="submit" />
    </Box>
  </Panel>;
};

export default Play;

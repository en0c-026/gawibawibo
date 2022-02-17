import { Box, Button, Text, TextInput } from 'grommet';
import React, { useState } from 'react';
import { utils } from 'near-api-js';
import { Options } from './Main'
import { sha256 } from 'js-sha256';

export const Play = ({ playTarget, contract, size }) => {
  const [blends, setBlends] = useState({ b1: "", b2: "", b3: "" });

  const handleClean = () => {
    setBlends({ b1: "", b2: "", b3: "" })
  };
  const handleSubmit = () => {
    const blend = sha256(Buffer.from([blends.b1, blends.b2, blends.b3]));
    contract.play_move({
      args: { id: playTarget.id, ha: blend },
      amount: playTarget.amount
    })
  };

  return <Box
    background='c1'
    align='center'
    height={{ min: '450px', max: '450px' }}
    pad={size === 'small' ? 'large' : 'medium'}
    gap={size === 'small' ? 'large' : 'medium'}
    border={{ color: 'c2' }}
    fill
  >
    <Text>Make a new move!</Text>
    <Text>Chooise a blend:</Text>
    <Options blends={blends} setBlends={setBlends} isLogged={true} />
    <Box direction='row' align='center' gap='small'>
      <Text margin={{ right: 'medium' }}>Amount</Text>
      <TextInput
        size='small'
        textAlign='end'
        value={utils.format.formatNearAmount(playTarget.amount)}
        disabled
      />
    </Box>
    <Box direction='row' alignSelf='end' gap='medium' pad={{ right: 'large' }}>
      <Button onClick={handleClean} label="clear" />
      <Button
        onClick={handleSubmit}
        disabled={blends.b1 !== '' && blends.b2 !== '' && blends.b3 !== '' ? false : true}
        label="submit"
      />
    </Box>
  </Box>;
};

export default Play;

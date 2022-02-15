import { Box, Button, Layer, List, RadioButtonGroup, ResponsiveContext, Text, TextInput, Card, Avatar } from 'grommet';
import { Close } from 'grommet-icons'
import React, { useContext, useState } from 'react';
import { Play } from './Play';
import { utils } from 'near-api-js';
import { useSnapshot } from 'valtio';
import { appState } from '../App';

export const Panel = ({ children }) => {
  return <Box background='c1' align='center' height={{ min: '450px', max: '450px' }} pad='medium' gap='medium' border={{ color: 'c2' }} fill>
    {children}
  </Box>
}

const Main = () => {
  const size = useContext(ResponsiveContext);
  const [blends, setBlends] = useState({ b1: "", b2: "", b3: "" });
  const [amount, setAmount] = useState("0");
  const [openPlay, setOpenPlay] = useState(false);
  const { wallet, accountId, isLogged, unplayedMoves, contract } = useSnapshot(appState);
  const [loading, setLoading] = useState(false);
  const [txUrl, setTxUrl] = useState('');
  const [playTarget, setPlayTarget] = useState({})

  const handlePlay = (id, amountPrize) => {
    setPlayTarget({ id: id, amount: amountPrize })
    setOpenPlay((value) => !value)
  };
  const handleCancel = (id) => {
    contract.cancel_move({ "id": id })
  };

  const handleClean = () => {
    setBlends({ b1: "", b2: "", b3: "" })
    setAmount("0")
  }

  const handleSubmit = () => {
    setLoading(true)
    const attachedAmount = utils.format.parseNearAmount(amount);
    const id = Math.floor(Math.random() * Date.now()) % 4294967296;
    contract.new_move({
      args: { id: id, blend: [blends.b1, blends.b2, blends.b3] },
      amount: attachedAmount
    }).then((result) => {
      if (result) {
        setLoading(false)
        setTxUrl("arre")
        console.log(txUrl, "existoso")
      }
    }
    );
  }



  return <Box gap='medium' align='center' pad='medium' direction={size !== 'large' ? 'column' : 'row'}>
    <Panel>
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
          disabled={isLogged ? false : true}
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
          disabled={isLogged ? false : true}
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
          disabled={isLogged ? false : true}
        />
      </Box>
      <Box direction='row' align='center' gap='small'>
        <Text margin={{ right: 'medium' }}>Amount</Text>
        <TextInput disabled={isLogged ? false : true} size='small' textAlign='end' value={amount} onChange={(e) => setAmount(e.target.value)} />
      </Box>
      <Box direction='row' alignSelf='end' gap='medium' pad={{ right: 'large' }}>
        <Button onClick={handleClean} label="clear" disabled={isLogged ? false : true} />
        <Button onClick={handleSubmit} label="submit" disabled={isLogged && amount > 0 && blends.b1 !== '' && blends.b2 !== '' && blends.b3 !== '' ? false : true} />
      </Box>
    </Panel>
    <Panel>
      <Text>unplayed moves</Text>
      <Box gap='medium' direction={size === 'small' ? 'column' : 'row'} align='center'>
        <Text size='small'>id</Text>
        <Text size='small'>owner</Text>
        <Text size='small'>prize</Text>
        <Text size='small'>play</Text>
      </Box>
      {
        unplayedMoves.length > 0 ?
          (
            <List data={unplayedMoves} primaryKey='id' paginate={{ size: 'small' }} step={5} action={(move, index) => (
              <ActionButton key={move.id} move={move} accountId={accountId} handleCancel={handleCancel} handlePlay={handlePlay} />
            )}>
              {(item) => (
                <Box key={item.id} gap='small' direction={size === 'small' ? 'column' : 'row'} align='center' justify='between'>
                  <Text size='small'>{item.id}</Text>
                  <Text size='small'>{item.owner}</Text>
                  <Text size='small'>{utils.format.formatNearAmount(item.prize)} NEAR</Text>
                </Box>
              )}
            </List>
          )
          :
          (
            <Card pad='small' size='small' background='c4' elevation='none' border>
              {
                isLogged ?
                  (<Text size='small'>There are no moves here.</Text>)
                  :
                  (<Text size='small'>Please login to view unplayed moves.</Text>)
              }
            </Card>
          )
      }
    </Panel>
    {
      openPlay && (
        <Layer position="center" margin="medium" responsive modal full="vertical" onClickOutside={handlePlay} onEsc={handlePlay}>
          <Box gap='medium' align='center' pad='medium'>
            <Close onClick={handlePlay} size='small' />
            <Play playTarget={playTarget} contract={contract} />
          </Box>
        </Layer>
      )
    }
  </Box >
};



const ActionButton = ({ move, accountId, handleCancel, handlePlay }) => {

  if (move.owner === accountId) {
    return <Button label='cancel' size='small' onClick={() => handleCancel(move.id)} />
  } else {
    return <Button label='play' size='small' onClick={() => handlePlay(move.id, move.prize)} />
  }

}

export default Main;
import {
  Box,
  Button,
  Checkbox,
  FormControlLabel,
  Grid,
  MenuItem,
  Stack,
  TextField,
  Typography,
} from '@mui/material';
import { PageSizes } from 'pdf-lib';
import { ChangeEvent, useEffect, useState } from 'react';
import { centerPdf } from './CenterPDF';

const App = () => {
  const [originalBytes, setOriginalBytes] = useState<ArrayBuffer>();
  const [originalUrl, setOriginalUrl] = useState('');
  const [centeredUrl, setCenteredUrl] = useState('');
  const [drawAlignment, setDrawAlignment] = useState(true);
  const [drawBorder, setDrawBorder] = useState(true);
  const [nudgeBorderHeight, setNudgeBorderHeight] = useState('7');
  const [nudgeBorderWidth, setNudgeBorderWidth] = useState('7');
  const [nudgeHeight, setNudgeHeight] = useState('0');
  const [nudgeWidth, setNudgeWidth] = useState('0');
  const [paperSize, setPaperSize] = useState('Letter');

  useEffect(() => {
    if (originalBytes) {
      centerPdf(originalBytes, {
        drawAlignment,
        drawBorder,
        nudgeBorderHeight: +nudgeBorderHeight,
        nudgeBorderWidth: +nudgeBorderWidth,
        nudgeWidth: +nudgeWidth,
        nudgeHeight: +nudgeHeight,
        paperSize: Object.entries(PageSizes).find(value => value[0] === paperSize)?.[1],
      }).then(centeredBytes => {
        const centeredBlob = new Blob([centeredBytes], { type: 'application/pdf' });
        setCenteredUrl(URL.createObjectURL(centeredBlob));
      });
    }
  }, [originalBytes, drawAlignment, drawBorder, nudgeBorderHeight, nudgeBorderWidth, nudgeWidth, nudgeHeight, paperSize]);

  const fileChangeHandler = async (event: ChangeEvent<HTMLInputElement>) => {
    const bytes = await event?.target?.files?.[0]?.arrayBuffer();
    if (bytes) {
      const originalBlob = new Blob([bytes], { type: 'application/pdf' });
      setOriginalBytes(bytes);
      setOriginalUrl(URL.createObjectURL(originalBlob));
    }
  };

  return (
    <Stack component='main' sx={{ height: '100%' }} spacing={2}>
      <Typography variant='h1'>Center PDF</Typography>
      <Grid container sx={{ paddingLeft: 2, gap: 2 }}>
        <Button variant='contained' component='label'>
          Upload
          <input hidden accept='application/pdf' onChange={fileChangeHandler} type='file' />
        </Button>
        <FormControlLabel label='Draw Alignment Corner' control={
          <Checkbox checked={drawAlignment} onChange={e => setDrawAlignment(e.target.checked)} />
        } />
        <FormControlLabel label='Draw Border' control={
          <Checkbox checked={drawBorder} onChange={e => setDrawBorder(e.target.checked)} />
        } />
        <TextField type='number' label='Nudge X' value={nudgeWidth}
                   onChange={e => setNudgeWidth(e.target.value)} />
        <TextField type='number' label='Nudge Y' value={nudgeHeight}
                   onChange={e => setNudgeHeight(e.target.value)} />
        <TextField type='number' label='Nudge Border X' value={nudgeBorderWidth}
                   onChange={e => setNudgeBorderWidth(e.target.value)} />
        <TextField type='number' label='Nudge Border Y' value={nudgeBorderHeight}
                   onChange={e => setNudgeBorderHeight(e.target.value)} />
        <TextField select label='Page Size' value={paperSize}
                   onChange={e => setPaperSize(e.target.value)}>
          {Object.keys(PageSizes).map((paperSizeName, i) =>
            <MenuItem key={i} value={paperSizeName}>{paperSizeName}</MenuItem>,
          )}
        </TextField>
      </Grid>
      <Grid sx={{ flexGrow: 1, padding: 2, paddingTop: 0 }} container gap={2}>
        <Grid item xs>
          {originalUrl &&
            <Box component='iframe' title='original' src={originalUrl}
                 sx={{ width: '100%', height: '100%' }}></Box>}
        </Grid>
        <Grid item xs>
          {centeredUrl &&
            <Box component='iframe' title='original' src={centeredUrl}
                 sx={{ width: '100%', height: '100%' }}></Box>}
        </Grid>
      </Grid>
    </Stack>
  );
};

export default App

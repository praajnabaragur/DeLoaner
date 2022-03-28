import * as React from "react";
import { styled } from "@mui/material/styles";
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell, { tableCellClasses } from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import Paper from "@mui/material/Paper";
import Typography from '@mui/material/Typography';

const StyledTableCell = styled(TableCell)(({ theme }) => ({
  [`&.${tableCellClasses.head}`]: {
    backgroundColor: theme.palette.common.black,
    color: theme.palette.common.white
  },
  [`&.${tableCellClasses.body}`]: {
    fontSize: 14
  }
}));

const StyledTableRow = styled(TableRow)(({ theme }) => ({
  "&:nth-of-type(odd)": {
    backgroundColor: theme.palette.action.hover
  },
  // hide last border
  "&:last-child td, &:last-child th": {
    border: 0
  }
}));

function createData(label: string, amount: string) {
  return { label, amount };
}

const rows = [
  createData("Amount paid back", '$25000'),
  createData("Amount interest", '$1000'),
  createData("Commission", '$500'),
  createData("Estimated Time", "3 months"),
];

export default function CustomizedTables() {
  return (
    <TableContainer component={Paper}>
      <Table
        sx={{ minWidth: 100, maxWidth: 400 }}
        aria-label="customized table"
      >
        <TableHead>
        <Typography variant="h4">Small Business info</Typography>
          
          <TableRow>
            <StyledTableCell align="center">label</StyledTableCell>
            <StyledTableCell align="center">amount</StyledTableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {rows.map((row) => (
            <StyledTableRow key={row.label}>
              <StyledTableCell component="th" scope="row">
                {row.label}
              </StyledTableCell>
              <StyledTableCell align="left">{row.amount}</StyledTableCell>
            </StyledTableRow>
          ))}
        </TableBody>
      </Table>
    </TableContainer>
  );
}
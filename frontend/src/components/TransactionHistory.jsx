import Card from "./Card";

export default function TransactionHistory({ transactions }) {
  if (!transactions) return null;
  return (
    <Card header={"Transaction History"}>
      <table>
        <thead>
          <tr>
            <th>Transaction Signature</th>
            <th>Block Time</th>
            <th>Slot</th>
            <th>Status</th>
          </tr>
        </thead>
        <tbody>
          {transactions.map((item) => (
            <tr key={item._id}>
              <td>{item._id}</td>
              <td>{item.block_time}</td>
              <td>{item.slot}</td>
              <td>{item.confirmation_status.toUpperCase()}</td>
            </tr>
          ))}
        </tbody>
      </table>
    </Card>
  );
}

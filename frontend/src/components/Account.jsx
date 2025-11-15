import Card from "./Card";

export default function Account({ data }) {
  if (!data) return null;
  return (
    <Card header="Account Overview">
      <table>
        <tbody>
          <tr>
            <td>Address</td>
            <td className="mono">{data._id}</td>
          </tr>
          <tr>
            <td>Balance (Lamports)</td>
            <td>
              <strong>◎{data.lamports ?? 0}</strong>
            </td>
          </tr>
          <tr>
            <td>Owner</td>
            <td>{data.owner}</td>
          </tr>
          <tr>
            <td>Executable</td>
            <td>{data.executable ? "Yes" : "No"}</td>
          </tr>
          <tr>
            <td>Allocated Data Size</td>
            <td>{data.data_length} byte(s)</td>
          </tr>
        </tbody>
      </table>
    </Card>
  );
}

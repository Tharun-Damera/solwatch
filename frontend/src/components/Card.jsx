export default function Card({ header, children }) {
  return (
    <div className="card">
      {header && <div className="card-header">{header}</div>}
      {children}
    </div>
  );
}

import {
  InputGroup,
  InputGroupAddon,
  InputGroupButton,
  InputGroupInput,
} from "@/components/ui/input-group";
import { SearchIcon } from "lucide-react";
import Logo from "./logo";

export default function NavBar() {
  return (
    <div className="nav-bar">
      <div className="title">
        <Logo />
        <span>
          <strong>SolWatch</strong> - <span>A Solana Lazy Indexer</span>
        </span>
      </div>
      <InputGroup className="search-bar">
        <InputGroupInput placeholder="Search for accounts and transactions" />
        <InputGroupAddon>
          <SearchIcon />
        </InputGroupAddon>
        <InputGroupAddon align="inline-end">
          <InputGroupButton>Search</InputGroupButton>
        </InputGroupAddon>
      </InputGroup>
    </div>
  );
}

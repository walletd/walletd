This doc explains non-fungible tokens (which conform to the ERC-721 standard)

# Interact with ERC-20 tokens
ERC-721​ is an EIP that details a non-fungible token standard for Ethereum.

## ERC-721 tokens implement the following functions and events

An ERC-721 token must implement the ERC721 interface and (optionally?) the ERC165 interface

```
interface ERC721 /* is ERC165 */ { 
    event Transfer(address indexed _from, address indexed _to, uint256 indexed _tokenId); 
    event Approval(address indexed _owner, address indexed _approved, uint256 indexed _tokenId); 
    event ApprovalForAll(address indexed _owner, address indexed _operator, bool _approved); 
    function balanceOf(address _owner) external view returns (uint256); 
    function ownerOf(uint256 _tokenId) external view returns (address); 
    function safeTransferFrom(address _from, address _to, uint256 _tokenId, bytes data) external payable; 
    function safeTransferFrom(address _from, address _to, uint256 _tokenId) external payable; 
    function transferFrom(address _from, address _to, uint256 _tokenId) external payable; 
    function approve(address _approved, uint256 _tokenId) external payable; 
    function setApprovalForAll(address _operator, bool _approved) external; 
    function getApproved(uint256 _tokenId) external view returns (address); 
    function isApprovedForAll(address _owner, address _operator) external view returns (bool); 
} 
interface ERC165 { 
    function supportsInterface(bytes4 interfaceID) external view returns (bool); 
} 
```

balanceOf: Finds the balance of an input address
ownerOf: Returns the owner address for a given token ID
safeTransferFrom: Transfers an NFT ownership from one address to another address, as opposed to transferFrom, and it also checks whether the recipient is a valid ERC-721 receiver address
transferFrom: Transfers ownership of an NFT
approve: Gives a given entity permission to transfer a token
setApprovalForAll: Controls the approval for a third party (operator) to manage all of msg.sender assets
getApproved: Gets the approved address for a single NFT
isApprovedForAll: Checks if approve for all addresses is allowed

Common methods shared between ERC20 and ERC721

balanceOf(owner) - Returns the account balance of another account with address owner. // common to erc20
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;
import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
contract EthereumBurn is Ownable {
    using SafeERC20 for IERC20;
    uint256 public constant MAX_TOKENS_TO_BURN = 150_000_000 ether;
    address public efiToken;
    address public enjToken;

    mapping(address => uint256) public totalBurnedAmount;
    event TokenBurned(address indexed user, address indexed token, uint256 amount);

    modifier supportedToken(address _token) {
        require(
            _token == efiToken || _token == enjToken,
            "Token not supported"
        );
        _;
    }
    modifier withinBurnLimit(address _token, uint256 _amount) {
        require(
            totalBurnedAmount[_token] + _amount <= MAX_TOKENS_TO_BURN,
            "Burned quantity exceeds cap"
        );
        _;
    }
    constructor(address _efiToken, address _enjToken) {
        efiToken = _efiToken;
        enjToken = _enjToken;
    }

    function burn(
        address _token,
        uint256 _amount
    ) external supportedToken(_token) withinBurnLimit(_token, _amount) {
        require(_amount > 0, "Burn amount cannot be zero");
        IERC20(_token).transferFrom(msg.sender, address(this), _amount);
        unchecked {
            totalBurnedAmount[_token] += _amount;
        }
        emit TokenBurned(msg.sender, _token, _amount);
    }

}
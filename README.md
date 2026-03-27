#  Hotel-Booking-Stellar

##  Description

**Hotel-Booking-Stellar** là một dự án ứng dụng công nghệ blockchain nhằm xây dựng hệ thống đặt phòng khách sạn minh bạch và an toàn. Dự án sử dụng nền tảng Stellar (Soroban smart contract) để lưu trữ và xác nhận trạng thái booking (đặt phòng) trên blockchain.

Mục tiêu của dự án:

* Tăng tính minh bạch trong quá trình đặt phòng và thanh toán
* Giảm sự phụ thuộc vào bên trung gian
* Giúp người dùng có thể kiểm tra trạng thái booking một cách đáng tin cậy

Lý do chọn ý tưởng này:

* Kết hợp kiến thức về **Web (React + NodeJS)** và **Blockchain**
* Ứng dụng thực tế cao (hotel booking là hệ thống phổ biến)
* Stellar phù hợp với các bài toán thanh toán nhanh, phí thấp

---

##  Features

###  Booking Management

* Tạo booking với `booking_id` và `amount`
* Lưu trạng thái booking trên blockchain

###  Payment Handling

* Cập nhật trạng thái từ `PENDING` → `PAID`
* Xác nhận thanh toán thông qua smart contract

###  Data Retrieval

* Lấy thông tin booking theo `booking_id`
* Truy xuất trực tiếp từ blockchain

###  Blockchain Integration

* Smart contract viết bằng **Rust (Soroban)**
* Deploy và tương tác trên **Stellar Testnet**

---

##  Smart Contract

Contract đã được deploy tại:

👉 https://stellar.expert/explorer/testnet/contract/CDDF2AB3TK5M7IF52MSQCX32QCZZ6KIFBFEPPELRCYBH3NCNZVWRUKUM?filter=history

###  Screenshot

![Contract Screenshot](https://github.com/user-attachments/assets/d69cfe69-7ea4-4350-9b30-b684e087aa93)

---

##  How It Works

### 🔄 Flow hệ thống:

1. Người dùng tạo booking
2. Smart contract lưu trạng thái `PENDING`
3. Người dùng thực hiện thanh toán
4. Backend (hoặc hệ thống) xác nhận giao dịch
5. Gọi smart contract → cập nhật `PAID`

---

## Future Scopes

Trong tương lai, dự án có thể được phát triển thêm:

*  Xác thực người dùng (wallet-based authentication)
*  Thanh toán thực bằng XLM hoặc token
*  Giao diện người dùng hoàn chỉnh (React UI)
*  Real-time update trạng thái booking
*  Hỗ trợ nhiều khách sạn (multi-hotel system)
*  Loyalty / Reward system bằng token
*  Dashboard quản lý cho admin

---

##  Profile

**Name:** Tra Nguyen Phuong
**Role:** Software Engineering Student


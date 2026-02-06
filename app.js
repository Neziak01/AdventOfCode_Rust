const cartPanel = document.getElementById("cart-panel");
const openCartButton = document.getElementById("open-cart");
const closeCartButton = document.getElementById("close-cart");
const cartItemsContainer = document.getElementById("cart-items");
const cartCount = document.getElementById("cart-count");
const cartTotal = document.getElementById("cart-total");

const cart = [];

const formatPrice = (value) =>
  new Intl.NumberFormat("fr-FR", {
    style: "currency",
    currency: "EUR",
    maximumFractionDigits: 0,
  }).format(value);

const renderCart = () => {
  cartItemsContainer.innerHTML = "";

  if (cart.length === 0) {
    cartItemsContainer.innerHTML = '<p class="empty">Votre panier est vide.</p>';
  } else {
    cart.forEach((item) => {
      const row = document.createElement("div");
      row.className = "cart-item";
      row.innerHTML = `
        <div>
          <span>${item.name}</span><br />
          <small>${formatPrice(item.price)}</small>
        </div>
        <strong>${item.quantity}x</strong>
      `;
      cartItemsContainer.appendChild(row);
    });
  }

  const total = cart.reduce((sum, item) => sum + item.price * item.quantity, 0);
  const totalQuantity = cart.reduce((sum, item) => sum + item.quantity, 0);
  cartTotal.textContent = formatPrice(total);
  cartCount.textContent = totalQuantity;
};

const toggleCart = (isOpen) => {
  cartPanel.classList.toggle("open", isOpen);
  cartPanel.setAttribute("aria-hidden", (!isOpen).toString());
};

openCartButton.addEventListener("click", () => toggleCart(true));
closeCartButton.addEventListener("click", () => toggleCart(false));

document.querySelectorAll(".add-to-cart").forEach((button) => {
  button.addEventListener("click", () => {
    const name = button.dataset.product;
    const price = Number(button.dataset.price);
    const existing = cart.find((item) => item.name === name);

    if (existing) {
      existing.quantity += 1;
    } else {
      cart.push({ name, price, quantity: 1 });
    }

    renderCart();
    toggleCart(true);
  });
});

renderCart();

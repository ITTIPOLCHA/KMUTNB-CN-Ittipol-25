class NavBar extends HTMLElement {
  constructor(){
    super();
  }

  connectedCallback() {
    this.innerHTML = `
    <div class="w3-top">
      <div class="w3-bar w3-red w3-card w3-left-align w3-large">
        <a class="w3-bar-item w3-button w3-hide-medium w3-hide-large w3-right w3-padding-large w3-hover-white w3-large w3-red" href="javascript:void(0);" onclick="myFunction()" title="Toggle Navigation Menu"><i class="fa fa-bars"></i></a>
        <a href="index.html" class="w3-bar-item w3-button w3-padding-large w3-white">Calculator</a>
        <a href="PIT.html" class="w3-bar-item w3-button w3-hide-small w3-padding-large w3-hover-white">PIT</a>
        <a href="CIT.html" class="w3-bar-item w3-button w3-hide-small w3-padding-large w3-hover-white">CIT</a>
        <a href="VAT.html" class="w3-bar-item w3-button w3-hide-small w3-padding-large w3-hover-white">VAT</a>
        <a href="History.html" class="w3-bar-item w3-button w3-hide-small w3-padding-large w3-hover-white">History</a>
        <a href="login.html" class="w3-bar-item w3-button w3-hide-small w3-padding-large w3-hover-lime float-end">Login</a>
      </div>
    
      <!-- Navbar on small screens -->
      <div id="navDemo" class="w3-bar-block w3-white w3-hide w3-hide-large w3-hide-medium w3-large">
        <a href="PIT.html" class="w3-bar-item w3-button w3-padding-large">PIT</a>
        <a href="CIT.html" class="w3-bar-item w3-button w3-padding-large">CIT</a>
        <a href="VAT.html" class="w3-bar-item w3-button w3-padding-large">VAT</a>
        <a href="History.html" class="w3-bar-item w3-button w3-padding-large">History</a>
        <a href="login.html" class="w3-bar-item w3-button w3-padding-large">Login</a>
      </div>
    </div>`
  }
}

customElements.define('navbar-component', NavBar)

class Footer extends HTMLElement {
  constructor(){
    super();
  }

  connectedCallback() {
    this.innerHTML = `
    <footer class="w3-container w3-padding-64 w3-center w3-opacity">  
      <p>อ้างอิง ข้อมูลจาก</p>
      <a href="https://tanateauditor.com/income-for-tax/">รายได้เท่าไหร่ต้องเสียภาษี? - tanate</a>
      <br>
      <a href="https://tanateauditor.com/pit/">ภาษีเงินได้บุคคลธรรมดาคำนวณอย่างไร - tanate</a>
    </footer>`
  }
}

customElements.define('footer-component', Footer)
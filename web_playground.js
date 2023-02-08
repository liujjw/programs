function sum(a, b) {
  return Promise(function (resolve, reject) {
    setTimeout(function() {
      resolve(a + b)
    }, 100)
  })
}


sum(1, 2).then(function(result) {

  return 
}).then().catch(function(err) {

})

document.write
document.cookie
localStorage.setItem('test', myJSONStr)
localStorage.getItem()

JSON.stringify(obj)

JSON.parse(str)


document.getElementById("form").addEventListener('submit', () => {
  let value = document.getElementById("name").value;
  if (value == "") {

  } else if (Number(value)) < 10) {

  }
})



<select>
  <option value="September">September</option> 
</select>

<nav><ul></ul></nav>

// TODO basic counter application by modifying inner html and event listener on the button 

import react from 'react'
import ReactDOM from 'react-dom'

// this is an example of classical inheritance, i never use protypical inheritance
// but basically you can create an object out of the base object or prototype in it
// Object.create(a javascript object)
class Hello extends React.Component {
  render () {
    return <div className='message-box'> Hello {this.props.name} </div>
  }
}

let Hello = () => {
  return <p></p >
}
//
// This JS is the one that I coded first
// It helped me to visualize the logical programming flow that the challenge required
//

// depth is given by the index of the letter
function getNodeDepth(node) {
    const letters = {
      a: 1,
      b: 2,
      c: 3,
      d: 4,
      e: 5,
      f: 6,
      g: 7,
      h: 8,
      i: 9,
      j: 10,
      k: 11,
      l: 12,
      m: 13,
      n: 14,
      o: 15,
      p: 16,
      q: 17,
      r: 18,
      s: 19,
      t: 20,
      u: 21,
      v: 22,
      w: 23,
      x: 24,
      y: 25,
      z: 26,
    }
    return letters[node]
  }
  
  function makeNode(_name, _depth, _branch) {
    let node = {
      name: _name,
      depth: _depth,
      branch: _branch,
    }
    return node
  }
  
  // alphabetical sort array DAG
  function sort(arr) {
    arr.sort(function (a, b) {
      if (a < b) {
        return -1
      }
      if (a > b) {
        return 1
      }
      return 0
    })
    return arr
  }
  
  // alphabetically sort DAG by name key in array
  function sortDagArr(arr) {
    arr.sort(function (a, b) {
      if (a.depth < b.depth) {
        return -1
      }
      if (a.depth > b.depth) {
        return 1
      }
      return 0
    })
    return arr
  }
  
  function makeDag(edgesArr) {
    // unitiliazed DAG
    let treeDag = {}
  
    for (var i = 0; i < edgesArr.length; i++) {
      let branchArr = edgesArr[i].split('-')
      let depth = getNodeDepth(branchArr[0])
  
      // check if the node has already been added
      if (branchArr[0] in treeDag) {
        // if it already exist, then we add the new branch to the node
        let node = treeDag[branchArr[0]]
        node.branch.push(branchArr[1])
        treeDag[branchArr[0]] = node
      } else {
        // node doesnt not exits, we initialize it, we send the branch as an array
        let node = makeNode(branchArr[0], depth, [branchArr[1]])
        treeDag[branchArr[0]] = node
      }
    }
    return treeDag
  }
  
  function stripDag(_delVer, _treeDag) {
    let nodeDelete = _treeDag[_delVer]
    let dagArr = []
    // the childs will reconnect to the parent node from the deleated vertex
    let parentNode = null
  
    // find parent key
    // 1) Convert dagDict to dagArr
    for (var key in _treeDag) {
      dagArr.push(_treeDag[key])
    }
    // 2) sort by depth
    dagArr = sortDagArr(dagArr)
  
    // 3) find first higher depth
    for (var i = 0; i < dagArr.length; i++) {
      //if the node is a child we reconnect it
      if (getNodeDepth(dagArr[i].name) < getNodeDepth(_delVer)) {
        // it will update until it wont succed, the dagArr has the depth increasing
        parentNode = dagArr[i]
      }
    }
  
    // Reconnect childs with parent node
    for (var i = 0; i < nodeDelete.branch.length; i++) {
      parentNode.branch.push(nodeDelete.branch[i])
    }
  
    // Remove all vertex
    for (var key in _treeDag) {
      let node = _treeDag[key]
      if (node.branch.includes(_delVer) === true) {
        let index = node.branch.indexOf(_delVer)
        if (index > -1) {
          node.branch.splice(index, 1)
        }
        _treeDag[key] = node
      }
    }
  
    // remove vertex node
    delete _treeDag[_delVer]
  
    return _treeDag
  }
  
  function printDag(_treeDag) {
    let arr = []
    for (var key in _treeDag) {
      let node = _treeDag[key]
      for (var i = 0; i < node.branch.length; i++) {
          let s = node.name+"-"+node.branch[i]
          arr.push(s)
      }
    }
    // alphabtecially order
    arr = sort(arr)
  
    return arr.toString()
  }
  
  function init() {
    let delVer = 'c'
    let dag = 'a-b,b-c,c-d'
    dag = dag.replace(/ /g, '')
    let edges = dag.split(',')
    edges = sort(edges)
    let treeDAG = makeDag(edges)
    treeDAG = stripDag(delVer, treeDAG)
    treeDAG = printDag(treeDAG)
  
    console.log(edges.toString())
    console.log(treeDAG)
  }
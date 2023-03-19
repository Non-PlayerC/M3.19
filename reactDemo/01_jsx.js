console.log("eiffel is running ..");

// 创建 jsx 元素
const element = {
    type: "h1",//创建的元素类型
    props: {
        // 储存类型的对象
        title: "foo",
        children: "Come",
    }
}
// 创建容器
const container = document.getElementById("root");

const node = document.createElement(element.type);// 创建元素
node["title"] = element.props.title;

const text = documen.createElement("");
text["nodeValue"] = element.props.children;

node.appendChild(text);
container.appendChild(node);
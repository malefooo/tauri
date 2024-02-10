<script lang="ts">
import {
  Document, FolderOpen, CloudUploadOutline, CubeSharp, BarChartOutline, CloudDownloadOutline,
  CodeDownloadSharp,ReloadCircleOutline,Options, AlertCircleOutline, GitPullRequest,RocketOutline,
  ReaderOutline, ResizeOutline,PlaySharp,
} from '@vicons/ionicons5'
import { defineComponent, ref } from 'vue'
import { repeat } from 'seemly'
import { TreeOption } from 'naive-ui'
import { invoke } from "@tauri-apps/api/tauri";
import { open } from '@tauri-apps/api/dialog';

const kmlInput = ref('')
const kmlOutput = ref('')

const photoHandleInput = ref('')
const photoHandleRadius = ref('')
const photoHandleOutput = ref('')

const lineTreeData = ref([''])
const photoCalculateTreeData = ref([''])

async function exportExcel() {
  const v = await invoke("kml_to_excel", {kmlFile: kmlInput.value, outputDir: kmlOutput.value});
  console.log(v);
}

async function kmlToTree() {
  const data = await invoke("kml_to_json", {kmlFile: kmlInput.value});
  console.log(data);
  const treeData = JSON.parse(data);
  lineTreeData.value = treeData;
}

async function openFileDialog() {
  try {
    const selected = await open({
      // 配置选项
      filters: [
        {
          name: 'KML Files',
          extensions: ['kml']
        }
      ]
    });
    console.log(selected);
    if (selected) {
      kmlInput.value = selected;
    }
  } catch (error) {
    console.error('文件选择错误', error);
  }
}

async function openFolderDialog() {
  try {
    const selected = await open({
      directory: true, // 设置为true来选择文件夹
      multiple: false  // 可以设置为true如果您想允许选择多个文件夹
    });
    if (selected) {
      console.log('选择的文件夹:', selected);
      kmlOutput.value = selected
    }
  } catch (error) {
    console.error('选择文件夹出错', error);
  }
}

export default defineComponent({
  components: {
  },
  setup () {
    return {
      Document, FolderOpen, CloudUploadOutline, CubeSharp, BarChartOutline, CloudDownloadOutline, CodeDownloadSharp,
      ReloadCircleOutline,Options, AlertCircleOutline, GitPullRequest,ReaderOutline, ResizeOutline, RocketOutline,PlaySharp,
      lineTreeData: lineTreeData,
      photoCalculateTreeData: photoCalculateTreeData,
      onlyAllowNumber: (value: string) => !value || /^\d+$/.test(value),
      exportExcel: exportExcel,
      photoHandleRadiusChange: (value: string) => {
        photoHandleRadius.value = value;
        console.log(photoHandleRadius.value);
      },
      photoHandleInput: photoHandleInput,
      photoHandleOutput: photoHandleOutput,
      openFileDialog: openFileDialog,
      openFolderDialog: openFolderDialog,
      kmlInput: kmlInput,
      kmlOutput: kmlOutput,
      kmlToTree: kmlToTree,

    }
  }
})

</script>

<template>

    <n-space vertical size="large">

      <n-layout class="bgc" >
        <n-layout-header class="head-font-color n-layout-header-standard">
          <n-icon :component="CubeSharp"></n-icon>  KML文件处理
        </n-layout-header>
        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="CloudUploadOutline"></n-icon>
                </template>
                导入
              </n-button>
            </n-gi>
            <n-gi :span="6">
              <n-input class="bgc"
                       :readonly="true"
                       v-model:value="kmlInput"
                       @click="openFileDialog"
                       style="
                       --n-text-decoration-color: burlywood;
                       --n-text-color: burlywood;
                       --n-color-focus: #2f2f2f;"
                       round
                       placeholder="文件: ../XXX/XXX.KML" />
            </n-gi>
          </n-grid>
        </n-layout-content>
        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="CloudDownloadOutline"></n-icon>
                </template>
                输出
              </n-button>
            </n-gi>
            <n-gi :span="6">
              <n-input class="bgc"
                       :readonly="true"
                       v-model:value="kmlOutput"
                       @click="openFolderDialog"
                       style="
                       --n-text-decoration-color: burlywood;
                       --n-text-color: burlywood;
                       --n-color-focus: #2f2f2f"
                       round placeholder="文件夹: ../XXX/XXX" />
            </n-gi>
          </n-grid>
        </n-layout-content>
        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="Options"></n-icon>
                </template>
                操作
              </n-button>
            </n-gi>
            <n-gi :span="3">
              <n-button @click="exportExcel" color="#DEB887FF" class="ngi-font-color-burlywood" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="CodeDownloadSharp"></n-icon>
                </template>
                导出excel
              </n-button>
            </n-gi>
            <n-gi :span="3">
              <n-button @click="kmlToTree" color="#DEB887FF" class="ngi-font-color-burlywood" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="ReloadCircleOutline"></n-icon>
                </template>
                导入线路数据
              </n-button>
            </n-gi>
          </n-grid>
        </n-layout-content>
        <n-layout-footer></n-layout-footer>
      </n-layout>

      <n-layout class="bgc">
        <n-layout-header class="head-font-color n-layout-header-standard">
          <n-icon :component="CubeSharp"></n-icon>  图片处理
        </n-layout-header>
        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi :span="7">
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="AlertCircleOutline"></n-icon>
                </template>
                说明:在点击计算后,会根据站点经纬度和半径画圆,然后判断照片的经纬度时候属于圆内
              </n-button>
            </n-gi>
          </n-grid>
        </n-layout-content>

        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">

            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="CloudUploadOutline"></n-icon>
                </template>
                导入
              </n-button>
            </n-gi>
            <n-gi :span="6">
              <n-input class="bgc"
                       :readonly="true"
                       v-model:value="photoHandleInput"
                       @click="openFolderDialog"
                       style="
                       --n-text-decoration-color: burlywood;
                       --n-text-color: burlywood;
                       --n-color-focus: #2f2f2f"
                       round
                       placeholder="文件夹: ../XXX/XXX" />
            </n-gi>
          </n-grid>
        </n-layout-content>

        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">

            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="ResizeOutline"></n-icon>
                </template>
                半径
              </n-button>
            </n-gi>
            <n-gi :span="6">
              <n-input class="bgc"
                       :allow-input="onlyAllowNumber"
                       style="
                       --n-text-decoration-color: burlywood;
                       --n-text-color: burlywood;
                       --n-color-focus: #2f2f2f" round placeholder="站点辐射的范围半径, 单位: 米" />
            </n-gi>
          </n-grid>
        </n-layout-content>

        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="CloudDownloadOutline"></n-icon>
                </template>
                输出
              </n-button>
            </n-gi>
            <n-gi :span="6">
              <n-input class="bgc"
                       :readonly="true"
                       v-model:value="photoHandleOutput"
                       @click="openFolderDialog"
                       style="
                       --n-text-decoration-color: burlywood;
                       --n-text-color: burlywood;
                       --n-color-focus: #2f2f2f"
                       round
                       placeholder="文件夹: ../XXX/XXX" />
            </n-gi>
          </n-grid>
        </n-layout-content>

        <n-layout-content content-style="padding: 10px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="Options"></n-icon>
                </template>
                操作
              </n-button>
            </n-gi>
            <n-gi :span="3">
              <n-button color="#DEB887FF" class="ngi-font-color-burlywood" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="RocketOutline"></n-icon>
                </template>
                计算
              </n-button>
            </n-gi>
            <n-gi :span="3">
              <n-button color="#DEB887FF" class="ngi-font-color-burlywood" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="PlaySharp"></n-icon>
                </template>
                开始
              </n-button>
            </n-gi>
          </n-grid>
        </n-layout-content>

        <n-layout-footer></n-layout-footer>
      </n-layout>

      <n-layout has-sider class="bgc">

        <n-layout class="bgc">
          <n-layout-header class="head-font-color n-layout-header-standard">
            <n-icon :component="BarChartOutline"></n-icon>  数据展示
          </n-layout-header>
          <n-layout-content content-style="padding: 10px;"  class="content-font-color">
            <n-grid x-gap="5" :cols="2" >
              <n-gi>
                <n-layout class="bgc">
                  <n-layout-header class="n-layout-header-internal">
                    <n-grid x-gap="10" :cols="3" >
                      <n-gi :span="3">
                        <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                          <template #icon>
                            <n-icon :component="CloudDownloadOutline"></n-icon>
                          </template>
                          导入的线路数据
                        </n-button>
                      </n-gi>
                    </n-grid>
                  </n-layout-header>
                  <n-layout-content style="height: 300px" content-style="padding: 10px;" class="content-font-color">
                    <n-tree
                        block-line
                        default-expand-all
                        :data="lineTreeData"
                        selectable
                        style="font-weight: 900; --n-node-text-color: burlywood"
                    />
                  </n-layout-content>
                  <n-layout-footer></n-layout-footer>
                </n-layout>
              </n-gi>
              <n-gi>
                <n-layout class="bgc">
                  <n-layout-header class="n-layout-header-internal">
                    <n-grid  x-gap="5" :cols="3">
                      <n-gi :span="3">
                        <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                          <template #icon>
                            <n-icon :component="ReaderOutline"></n-icon>
                          </template>
                          导入的照片数计算结果
                        </n-button>
                      </n-gi>
                    </n-grid>
                  </n-layout-header>
                  <n-layout-content style="height: 300px" content-style="padding: 10px;"  class="content-font-color">
                    <n-tree
                        block-line
                        default-expand-all
                        :data="photoCalculateTreeData"
                        selectable
                        style="font-weight: 900; --n-node-text-color: burlywood"
                    />
                  </n-layout-content>
                  <n-layout-footer></n-layout-footer>
                </n-layout>
              </n-gi>
            </n-grid>

          </n-layout-content>
          <n-layout-footer></n-layout-footer>
        </n-layout>
      </n-layout>

    </n-space>

</template>

<style scoped>

.center-text {
  text-align: center;
}

.center-text-vertical {
  display: flex;
  justify-content: center; /* 水平居中 */
  align-items: center;     /* 垂直居中 */
  height: 100px;           /* 或者根据需要设置高度 */
}

.bgc {
  background-color: #2f2f2f
}

.n-layout-header-standard {
  background: rgba(128, 128, 128, 0.2);
  padding: 2px;
}

.n-layout-header-internal {
  background: rgba(128, 128, 128, 0.4);
  padding: 2px;
}

.n-layout-footer {
  background: rgba(128, 128, 128, 0.2);
  padding: 2px;
}

.n-layout-sider {
  background: rgba(128, 128, 128, 0.3);
}

.n-layout-content {
  background: rgba(128, 128, 128, 0.4);
}

.head-font-color {
  color: wheat;
}

.content-font-color {
  color: burlywood;
}

.wheat1 {
  height: 30px;
  background-color: wheat;
}
.burlywood1 {
  height: 30px;
  background-color: burlywood;
}
.ngi-font-color {
  color: #2f2f2f;
}
.ngi-font-color-burlywood {
  color: burlywood;
}
</style>
<script setup lang="ts">
import {
  Document, FolderOpen, CloudUploadOutline, CubeSharp, BarChartOutline, CloudDownloadOutline,
  CodeDownloadSharp,ReloadCircleOutline,Options, AlertCircleOutline, GitPullRequest,
  PlaySharp,ReaderOutline
} from '@vicons/ionicons5'

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
            <n-gi>
              <n-button color="#DEB887FF" class="ngi-font-color" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="Document"></n-icon>
                </template>
                文件
              </n-button>
            </n-gi>
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="FolderOpen"></n-icon>
                </template>
                文件夹
              </n-button>
            </n-gi>
            <n-gi :span="4">
              <n-input class="bgc" disabled readonly round placeholder="文件: ../XXX/XXX.KML 或 文件夹: ../XXX/XXX" />
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
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="FolderOpen"></n-icon>
                </template>
                文件夹
              </n-button>
            </n-gi>
            <n-gi :span="5">
              <n-input class="bgc" disabled readonly round placeholder="文件夹: ../XXX/XXX" />
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
            <n-gi :span="2">
              <n-button color="#DEB887FF" class="ngi-font-color" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="CodeDownloadSharp"></n-icon>
                </template>
                导出excel
              </n-button>
            </n-gi>
            <n-gi :span="2">
              <n-button color="#DEB887FF" class="ngi-font-color" style="width: 100%" ghost round>
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
          <n-icon :component="CubeSharp"></n-icon>  导入线路数据
        </n-layout-header>
        <n-layout-content content-style="padding: 12px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="CloudUploadOutline"></n-icon>
                </template>
                导入
              </n-button>
            </n-gi>
            <n-gi>
              <n-button color="#DEB887FF" class="ngi-font-color" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="Document"></n-icon>
                </template>
                文件
              </n-button>
            </n-gi>
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="FolderOpen"></n-icon>
                </template>
                文件夹
              </n-button>
            </n-gi>
            <n-gi :span="4">
              <n-input class="bgc" disabled readonly round placeholder="文件: ../XXX/XXX.KML 或 文件夹: ../XXX/XXX" />
            </n-gi>
          </n-grid>
        </n-layout-content>
        <n-layout-footer></n-layout-footer>
      </n-layout>


      <n-layout class="bgc">
        <n-layout-header class="head-font-color n-layout-header-standard">
          <n-icon :component="CubeSharp"></n-icon>  给定站点半径
        </n-layout-header>
        <n-layout-content content-style="padding: 12px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi :span="6">
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="AlertCircleOutline"></n-icon>
                </template>
                说明:以站点为圆心画圆,根据照片经纬度判断时候属于其圆内
              </n-button>
            </n-gi>
            <n-gi>
              <n-input class="bgc" round placeholder="..米" />
            </n-gi>
          </n-grid>
        </n-layout-content>
        <n-layout-footer></n-layout-footer>
      </n-layout>

      <n-layout class="bgc">
        <n-layout-header class="head-font-color n-layout-header-standard">
          <n-icon :component="CubeSharp"></n-icon>  导入图片
        </n-layout-header>
        <n-layout-content content-style="padding: 12px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="CloudUploadOutline"></n-icon>
                </template>
                普通/红外
              </n-button>
            </n-gi>
            <n-gi>
              <n-input class="bgc" disabled readonly round placeholder="0/0" />
            </n-gi>
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="FolderOpen"></n-icon>
                </template>
                文件夹
              </n-button>
            </n-gi>
            <n-gi :span="4">
              <n-input class="bgc" disabled readonly round placeholder="文件: ../XXX/XXX.KML 或 文件夹: ../XXX/XXX" />
            </n-gi>
          </n-grid>
        </n-layout-content>
        <n-layout-footer></n-layout-footer>
      </n-layout>


      <n-layout class="bgc">
        <n-layout-header class="head-font-color n-layout-header-standard">
          <n-icon :component="CubeSharp"></n-icon>  输出地址
        </n-layout-header>
        <n-layout-content content-style="padding: 12px;" class="content-font-color">
          <n-grid x-gap="5" :cols="7">
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                <template #icon>
                  <n-icon :component="CloudDownloadOutline"></n-icon>
                </template>
                输出
              </n-button>
            </n-gi>
            <n-gi>
              <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%" ghost round>
                <template #icon>
                  <n-icon :component="FolderOpen"></n-icon>
                </template>
                文件夹
              </n-button>
            </n-gi>
            <n-gi :span="5">
              <n-input class="bgc" disabled readonly round placeholder="文件夹: ../XXX/XXX" />
            </n-gi>
          </n-grid>
        </n-layout-content>
        <n-layout-footer></n-layout-footer>
      </n-layout>


      <n-layout has-sider class="bgc">

        <n-layout class="bgc">
          <n-layout-header class="head-font-color n-layout-header-standard">
            <n-icon :component="BarChartOutline"></n-icon>  线路数据展示
          </n-layout-header>
          <n-layout-content content-style="padding: 10px;"  class="content-font-color">

            <n-grid x-gap="5" :cols="2" >
              <n-gi>
                <n-layout class="bgc">
                  <n-layout-header class="n-layout-header-internal">
                    <n-grid x-gap="10" :cols="3" >
                      <n-gi :span="2">
                        <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                          <template #icon>
                            <n-icon :component="CloudDownloadOutline"></n-icon>
                          </template>
                          要导入的数据
                        </n-button>
                      </n-gi>
                      <n-gi>
                        <n-button color="#DEB887FF" class="ngi-font-color" style="width: 100%" round>
                          <template #icon>
                            <n-icon :component="GitPullRequest"></n-icon>
                          </template>
                          覆盖
                        </n-button>
                      </n-gi>
                    </n-grid>
                  </n-layout-header>
                  <n-layout-content style="height: 300px" content-style="padding: 10px;" class="content-font-color">

                  </n-layout-content>
                  <n-layout-footer></n-layout-footer>
                </n-layout>
              </n-gi>
              <n-gi>
                <n-layout class="bgc">
                  <n-layout-header class="n-layout-header-internal">
                    <n-grid  x-gap="5" :cols="3">
                      <n-gi :span="2">
                        <n-button color="#F5DEB3FF" class="ngi-font-color" style="width: 100%; font-weight: 900" disabled round>
                          <template #icon>
                            <n-icon :component="ReaderOutline"></n-icon>
                          </template>
                          已存在的数据
                        </n-button>
                      </n-gi>
                      <n-gi>
                        <n-button color="#DEB887FF" class="ngi-font-color" style="width: 100%" round>
                          <template #icon>
                            <n-icon :component="PlaySharp"></n-icon>
                          </template>
                          开始
                        </n-button>
                      </n-gi>
                    </n-grid>
                  </n-layout-header>
                  <n-layout-content style="height: 300px" content-style="padding: 10px;"  class="content-font-color">
                    <template>

                    </template>
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

</style>
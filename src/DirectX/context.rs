use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Direct3D::D3D_FEATURE_LEVEL_11_0;
use windows::Win32::Graphics::Direct3D12::{
    D3D12CreateDevice, ID3D12CommandAllocator, ID3D12CommandList, ID3D12CommandQueue,
    ID3D12DescriptorHeap, ID3D12Device, ID3D12Resource,
};
use windows::Win32::Graphics::Dxgi::{IDXGIAdapter1, IDXGIFactory6, IDXGISwapChain1};
#[derive(Debug, Default)]
pub struct Context {
    pub factory: Option<IDXGIFactory6>,
    pub device: Option<ID3D12Device>,
    pub command_queue: Option<ID3D12CommandQueue>,
    pub swap_chain: Option<IDXGISwapChain1>,
    pub back_buffers: Vec<ID3D12Resource>,
    pub command_list: Option<ID3D12CommandList>,
    pub command_allocator: Option<ID3D12CommandAllocator>,
    pub RTVDescriptorHeap: Option<ID3D12DescriptorHeap>,
    pub hwnd: Option<HWND>,
    pub window_rect: Option<RECT>,
    pub RTV_descriptor_size: u32,
    pub current_back_buffer_index: u32,
}

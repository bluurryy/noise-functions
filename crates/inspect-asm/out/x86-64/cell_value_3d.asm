inspect_asm::cell_value_3d:
	push rbp
	push r15
	push r14
	push r13
	push r12
	push rbx
	movss xmm1, dword ptr [rdi]
	xorps xmm2, xmm2
	cmpnless xmm2, xmm1
	movss xmm0, dword ptr [rip + .LCPI10_0]
	movaps xmm3, xmm2
	andps xmm3, xmm0
	movss xmm4, dword ptr [rip + .LCPI10_1]
	andnps xmm2, xmm4
	orps xmm2, xmm3
	addss xmm2, xmm1
	cvttss2si r8d, xmm2
	movss xmm5, dword ptr [rip + .LCPI10_2]
	ucomiss xmm2, xmm5
	mov eax, 2147483647
	cmova r8d, eax
	xor edx, edx
	ucomiss xmm2, xmm2
	cmovp r8d, edx
	movss xmm2, dword ptr [rdi + 4]
	xorps xmm6, xmm6
	cmpnless xmm6, xmm2
	movaps xmm3, xmm6
	andps xmm3, xmm0
	andnps xmm6, xmm4
	orps xmm6, xmm3
	addss xmm6, xmm2
	cvttss2si ecx, xmm6
	ucomiss xmm6, xmm5
	cmova ecx, eax
	movss xmm3, dword ptr [rdi + 8]
	ucomiss xmm6, xmm6
	cmovp ecx, edx
	xorps xmm6, xmm6
	cmpnless xmm6, xmm3
	andps xmm0, xmm6
	andnps xmm6, xmm4
	orps xmm6, xmm0
	addss xmm6, xmm3
	cvttss2si edi, xmm6
	ucomiss xmm6, xmm5
	cmova edi, eax
	ucomiss xmm6, xmm6
	cmovp edi, edx
	xorps xmm0, xmm0
	lea r10d, [r8 - 1]
	inc r8d
	cmp r10d, r8d
	jle .LBB_1
.LBB_20:
	pop rbx
	pop r12
	pop r13
	pop r14
	pop r15
	pop rbp
	ret
.LBB_1:
	lea r15d, [rcx - 1]
	inc ecx
	cmp r15d, ecx
	jle .LBB_4
.LBB_2:
	xor eax, eax
	cmp r10d, r8d
	setl cl
	jge .LBB_20
	mov al, cl
	add r10d, eax
	cmp r10d, r8d
	jle .LBB_2
	jmp .LBB_20
.LBB_4:
	lea esi, [rdi - 1]
	inc edi
	cmp esi, edi
	jle .LBB_5
.LBB_15:
	mov eax, r15d
.LBB_16:
	xor edx, edx
	cmp eax, ecx
	setl sil
	jge .LBB_18
	mov dl, sil
	add eax, edx
	cmp eax, ecx
	jle .LBB_16
.LBB_18:
	xor eax, eax
	cmp r10d, r8d
	setl dl
	jge .LBB_20
	mov al, dl
	add r10d, eax
	cmp r10d, r8d
	jle .LBB_15
	jmp .LBB_20
.LBB_5:
	imul r9d, r10d, 501125321
	imul eax, r15d, 1136930381
	mov dword ptr [rsp - 24], eax
	imul eax, esi, 1720413743
	mov dword ptr [rsp - 12], eax
	xor r11d, r11d
	movss xmm5, dword ptr [rip + .LCPI10_3]
	lea r14, [rip + .L__unnamed__0]
	movss xmm0, dword ptr [rip + .LCPI10_4]
	movaps xmm4, xmmword ptr [rip + .LCPI10_5]
	mov qword ptr [rsp - 8], r8
	mov dword ptr [rsp - 20], r15d
.LBB_6:
	mov dword ptr [rsp - 16], r10d
	xorps xmm6, xmm6
	cvtsi2ss xmm6, r10d
	subss xmm6, xmm1
	mov ebp, dword ptr [rsp - 24]
.LBB_7:
	mov r12d, ebp
	xorps xmm7, xmm7
	cvtsi2ss xmm7, r15d
	xor r12d, r9d
	subss xmm7, xmm2
	mov r13d, dword ptr [rsp - 12]
	mov ebx, esi
	mov r8d, esi
.LBB_8:
	movaps xmm8, xmm5
	xor esi, esi
	cmp r8d, edi
	setl al
	mov edx, r12d
	xor edx, r13d
	imul edx, edx, 668265261
	mov r10d, edx
	and r10d, 1020
	movss xmm9, dword ptr [r14 + 4*r10 + 4]
	mulss xmm9, xmm0
	xorps xmm5, xmm5
	cvtsi2ss xmm5, r8d
	addss xmm9, xmm7
	subss xmm5, xmm3
	mulss xmm9, xmm9
	movss xmm10, dword ptr [r14 + 4*r10 + 8]
	movss xmm11, dword ptr [r14 + 4*r10]
	unpcklps xmm11, xmm10
	mulps xmm11, xmm4
	movlhps xmm5, xmm6
	shufps xmm5, xmm6, 226
	addps xmm5, xmm11
	mulps xmm5, xmm5
	addss xmm9, xmm5
	shufps xmm5, xmm5, 85
	addss xmm5, xmm9
	ucomiss xmm8, xmm5
	cmova r11d, edx
	minss xmm5, xmm8
	cmp r8d, edi
	jge .LBB_10
	mov sil, al
	add r8d, esi
	add r13d, 1720413743
	cmp r8d, edi
	jle .LBB_8
.LBB_10:
	xor eax, eax
	cmp r15d, ecx
	setl dl
	mov esi, ebx
	jge .LBB_12
	mov al, dl
	add r15d, eax
	add ebp, 1136930381
	cmp r15d, ecx
	jle .LBB_7
.LBB_12:
	xor eax, eax
	mov r8, qword ptr [rsp - 8]
	mov r10d, dword ptr [rsp - 16]
	cmp r10d, r8d
	setl dl
	mov r15d, dword ptr [rsp - 20]
	jge .LBB_14
	mov al, dl
	add r10d, eax
	add r9d, 501125321
	cmp r10d, r8d
	jle .LBB_6
.LBB_14:
	xorps xmm0, xmm0
	cvtsi2ss xmm0, r11d
	mulss xmm0, dword ptr [rip + .LCPI10_6]
	jmp .LBB_20
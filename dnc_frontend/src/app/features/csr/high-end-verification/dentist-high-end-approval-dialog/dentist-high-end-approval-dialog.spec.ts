import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DentistHighEndApprovalDialog } from './dentist-high-end-approval-dialog';

describe('DentistHighEndApprovalDialog', () => {
  let component: DentistHighEndApprovalDialog;
  let fixture: ComponentFixture<DentistHighEndApprovalDialog>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DentistHighEndApprovalDialog]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DentistHighEndApprovalDialog);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

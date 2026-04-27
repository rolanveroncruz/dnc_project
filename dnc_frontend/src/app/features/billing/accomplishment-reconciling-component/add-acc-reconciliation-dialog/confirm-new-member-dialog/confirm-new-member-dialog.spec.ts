import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ConfirmNewMemberDialog } from './confirm-new-member-dialog';

describe('ConfirmNewMemberDialog', () => {
  let component: ConfirmNewMemberDialog;
  let fixture: ComponentFixture<ConfirmNewMemberDialog>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [ConfirmNewMemberDialog]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ConfirmNewMemberDialog);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

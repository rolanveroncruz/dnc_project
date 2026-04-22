import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AddAccReconciliationDialog } from './add-acc-reconciliation-dialog';

describe('AddAccReconciliationDialog', () => {
  let component: AddAccReconciliationDialog;
  let fixture: ComponentFixture<AddAccReconciliationDialog>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [AddAccReconciliationDialog]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AddAccReconciliationDialog);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});

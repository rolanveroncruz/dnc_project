import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DataTableWithSelectComponent } from './data-table-with-select-component';

describe('DataTableWithSelectComponent', () => {
  let component: DataTableWithSelectComponent;
  let fixture: ComponentFixture<DataTableWithSelectComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [DataTableWithSelectComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DataTableWithSelectComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
